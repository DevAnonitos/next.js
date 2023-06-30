pub(crate) mod css_client_reference;
pub(crate) mod ecmascript_client_reference;
pub(crate) mod visit_client_reference;

pub use css_client_reference::css_client_reference_asset::CssClientReferenceAssetVc;
pub use ecmascript_client_reference::{
    ecmascript_client_reference_asset::EcmascriptClientReferenceAssetVc,
    ecmascript_client_reference_transition::NextEcmascriptClientReferenceTransitionVc,
};
pub use visit_client_reference::{
    ClientReference, ClientReferenceType, ClientReferencesByEntry, ClientReferencesByEntryVc,
};
