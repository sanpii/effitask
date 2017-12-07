#![feature(proc_macro)]
#![feature(slice_concat_ext)]

extern crate chrono;
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

mod agenda;
mod application;
mod done;
mod inbox;
mod tasks;
mod widgets;

fn main()
{
    ::env_logger::init()
        .unwrap();

    let todo_file = match ::std::env::var("TODO_FILE") {
        Ok(todo_file) => todo_file,
        Err(_) => panic!("Launch this program via todo.sh"),
    };

    let done_file = match ::std::env::var("DONE_FILE") {
        Ok(done_file) => done_file,
        Err(_) => panic!("Launch this program via todo.sh"),
    };

    let tasks = ::tasks::List::from_files(
        ::std::path::Path::new(&todo_file),
        ::std::path::Path::new(&done_file)
    );

    application::Widget::run(tasks)
        .unwrap();
}
