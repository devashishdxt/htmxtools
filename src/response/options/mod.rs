//! HTMX response header options.
mod events;
mod location_options;
mod swap_option;

pub use self::{
    events::{Event, Events},
    location_options::LocationOptions,
    swap_option::SwapOption,
};
