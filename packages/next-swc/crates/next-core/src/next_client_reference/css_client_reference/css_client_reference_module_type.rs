use anyhow::{bail, Result};
use turbo_tasks::Value;
use turbopack_binding::turbopack::{
    core::{
        asset::AssetVc,
        reference_type::{CssReferenceSubType, ReferenceType},
        resolve::ModulePartVc,
    },
    turbopack::{
        css::chunk::CssChunkPlaceableVc,
        module_options::{CustomModuleType, CustomModuleTypeVc},
        transition::{Transition, TransitionVc},
        ModuleAssetContextVc,
    },
};

use super::css_client_reference_asset::CssClientReferenceAssetVc;

#[turbo_tasks::value]
pub struct CssClientReferenceModuleType {
    client_transition: TransitionVc,
}

#[turbo_tasks::value_impl]
impl CssClientReferenceModuleTypeVc {
    #[turbo_tasks::function]
    pub fn new(client_transition: TransitionVc) -> Self {
        CssClientReferenceModuleType { client_transition }.cell()
    }
}

#[turbo_tasks::value_impl]
impl CustomModuleType for CssClientReferenceModuleType {
    #[turbo_tasks::function]
    async fn create_module(
        &self,
        source: AssetVc,
        context: ModuleAssetContextVc,
        _part: Option<ModulePartVc>,
    ) -> Result<AssetVc> {
        let client_asset = self.client_transition.process(
            source,
            context,
            Value::new(ReferenceType::Css(CssReferenceSubType::Internal)),
        );

        let Some(client_module) = CssChunkPlaceableVc::resolve_from(&client_asset).await? else {
            bail!("client asset is not CSS chunk placeable");
        };

        Ok(CssClientReferenceAssetVc::new(client_module).into())
    }
}
