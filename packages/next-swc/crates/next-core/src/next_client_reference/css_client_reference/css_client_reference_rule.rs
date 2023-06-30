use turbopack_binding::turbopack::{
    core::reference_type::{CssReferenceSubType, ReferenceType},
    turbopack::{
        module_options::{ModuleRule, ModuleRuleCondition, ModuleRuleEffect, ModuleType},
        transition::TransitionVc,
    },
};

use super::css_client_reference_module_type::CssClientReferenceModuleTypeVc;

pub(crate) fn get_next_css_client_reference_transforms_rule(
    client_transition: TransitionVc,
) -> ModuleRule {
    let module_type = CssClientReferenceModuleTypeVc::new(client_transition);

    ModuleRule::new(
        // Override the default module type for CSS assets. Instead, they will go through the
        // custom CSS client reference module type, which will:
        // 1. Chunk them through the client chunking context.
        // 2. Propagate them to the client references manifest.
        ModuleRuleCondition::all(vec![
            ModuleRuleCondition::ReferenceType(ReferenceType::Css(CssReferenceSubType::Internal)),
            ModuleRuleCondition::any(vec![ModuleRuleCondition::ResourcePathEndsWith(
                ".css".to_string(),
            )]),
        ]),
        vec![ModuleRuleEffect::ModuleType(ModuleType::Custom(
            module_type.into(),
        ))],
    )
}
