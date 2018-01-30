thread_local!(
    pub static PREFERENCES: ::std::cell::RefCell<super::Preferences> = ::std::cell::RefCell::new(super::Preferences::new());
    pub static TASKS: ::std::cell::RefCell<::tasks::List> = ::std::cell::RefCell::new(::tasks::List::new());
);

pub fn preferences() -> super::Preferences
{
    let mut preferences = super::Preferences::new();

    PREFERENCES.with(|p| {
        preferences = p.borrow().clone();
    });

    preferences
}

pub fn tasks() -> ::tasks::List
{
    let mut list = ::tasks::List::new();

    TASKS.with(|t| {
        list = t.borrow().clone();
    });

    list
}

pub fn add_task(text: &str) -> Result<(), String>
{
    let mut result = Ok(());

    TASKS.with(|t| {
        result = t.borrow_mut().add(text);
    });

    result
}
