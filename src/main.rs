#![feature(proc_macro)]
#![feature(slice_concat_ext)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate cairo;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate gdk;
extern crate gdk_sys;
extern crate glib;
extern crate gtk;
extern crate pulldown_cmark;
extern crate rand;
extern crate regex;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
extern crate todo_txt;
extern crate xdg;

use relm::Widget;

mod agenda;
mod application;
mod done;
mod edit;
mod flag;
mod inbox;
mod logger;
mod search;
mod tasks;
mod widgets;

fn main()
{
    if ::std::env::args().nth(1) == Some("usage".to_owned()) {
        usage(&::std::env::args().nth(0).unwrap());

        ::std::process::exit(0);
    }

    ::application::Widget::run(())
        .unwrap();
}

fn usage(program: &str)
{
    let path = ::std::path::Path::new(&program);

    println!("    {}", path.file_name().unwrap().to_str().unwrap());
    println!("      Launch focus graphical interface");
}
