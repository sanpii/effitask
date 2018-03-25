mod globals;
mod preferences;
mod widget;

pub use self::globals::add_task;
pub use self::globals::preferences;
pub use self::globals::tasks;
pub use self::preferences::Preferences;
pub use self::widget::Widget;

pub const NAME: &str = "Effitask";
