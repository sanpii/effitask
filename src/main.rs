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

    if ::std::env::args().nth(1) == Some("usage".to_owned()) {
        usage(::std::env::args().nth(0).unwrap());

        ::std::process::exit(0);
    }

    ::application::Widget::run(())
        .unwrap();
}

fn usage(program: String)
{
    let path = ::std::path::Path::new(&program);

    println!("    {}", path.file_name().unwrap().to_str().unwrap());
    println!("      Launch focus graphical interface");
}
