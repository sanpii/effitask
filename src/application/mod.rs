mod globals;
mod preferences;
mod widget;
pub mod env;

pub use globals::preferences::get as preferences;
pub use globals::tasks::add as add_task;
pub use globals::tasks::get as tasks;
pub use preferences::Preferences;
pub use widget::Widget;

pub const NAME: &str = env!("CARGO_PKG_NAME");
