//! HTMX request headers.
mod hx_boosted;
mod hx_current_url;
mod hx_history_restore_request;
#[cfg(feature = "preload")]
mod hx_preload;
mod hx_prompt;
mod hx_request;
mod hx_target;
mod hx_trigger;
mod hx_trigger_name;

#[cfg(feature = "preload")]
#[cfg_attr(docsrs, doc(cfg(feature = "preload")))]
pub use self::hx_preload::HxPreload;
pub use self::{
    hx_boosted::HxBoosted, hx_current_url::HxCurrentUrl,
    hx_history_restore_request::HxHistoryRestoreRequest, hx_prompt::HxPrompt,
    hx_request::HxRequest, hx_target::HxTarget, hx_trigger::HxTrigger,
    hx_trigger_name::HxTriggerName,
};
