#![feature(proc_macro)]

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
    application::Widget::run(())
        .unwrap();
}
