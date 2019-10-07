mod globals;
mod preferences;
mod widget;

pub use self::globals::preferences::get as preferences;
pub use self::globals::tasks::add as add_task;
pub use self::globals::tasks::get as tasks;
pub use self::preferences::Preferences;
pub use self::widget::Widget;

pub const NAME: &str = "Effitask";
