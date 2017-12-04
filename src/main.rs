#![feature(proc_macro)]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
extern crate todo_txt;

use relm::Widget;

mod application;
mod inbox;
mod tasks;

fn main()
{
    ::env_logger::init()
        .unwrap();

    let tasks = ::tasks::List::from_files(
        ::std::path::Path::new("/home/sanpi/.local/opt/share/todo/todo.txt"),
        ::std::path::Path::new("/home/sanpi/.local/opt/share/todo/done.txt")
    );

    application::Widget::run(tasks)
        .unwrap();
}
