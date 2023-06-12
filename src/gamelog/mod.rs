use rltk::RGB;
mod logstore;
use logstore::*;
pub use logstore::{clear_log, log_display, clone_log, restore_log};
mod builder;
pub use builder::*;
mod events;
pub use events::{record_event, clear_events, get_event_count, clone_events, load_events};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LogFragment {
    pub color : RGB,
    pub text : String
}