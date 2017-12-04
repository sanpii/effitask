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

fn main()
{
    ::env_logger::init()
        .unwrap();

    application::Widget::run(tasks)
        .unwrap();
}
