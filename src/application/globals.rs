pub mod preferences {
    use crate::application::Preferences;

    lazy_static::lazy_static! {
        static ref PREFERENCES: std::sync::RwLock<Preferences> =
            std::sync::RwLock::new(Preferences::new());
    }

    pub fn get() -> Preferences {
        PREFERENCES.read()
            .expect("Unable to rlock preferences")
            .clone()
    }

    pub fn replace(new: Preferences) {
        let mut preferences = PREFERENCES.write()
            .expect("Unable to wlock preferences");

        *preferences = new;
    }
}

pub mod tasks {
    use crate::tasks::List;

    lazy_static::lazy_static! {
        static ref TASKS: std::sync::RwLock<List> =
            std::sync::RwLock::new(List::new());
    }

    pub fn get() -> List {
        TASKS.read()
            .expect("Unable to rlock tasks")
            .clone()
    }

    pub fn add(text: &str) -> Result<(), String> {
        let mut tasks = TASKS.write()
            .expect("Unable to wlock tasks");

        (*tasks).add(text)
    }


    pub fn replace(new: List) {
        let mut tasks = TASKS.write()
            .expect("Unable to wlock tasks");

        *tasks = new;
    }
}
