#[derive(Clone)]
pub struct Preferences {
    pub defered: bool,
    pub done: bool,
}

impl Preferences
{
    pub fn new() -> Self
    {
        Self {
            defered: false,
            done: false,
        }
    }
}
