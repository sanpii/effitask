#![warn(rust_2018_idioms)]

use relm::Widget;

mod add;
mod agenda;
mod application;
mod date;
mod done;
mod edit;
mod flag;
mod inbox;
mod logger;
mod search;
mod tasks;
mod widgets;

fn main() {
    let environment = application::environment::EffiEnvironment::new();
    human_panic::setup_panic!();

    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    if std::env::args().nth(1) == Some("usage".to_string()) {
        usage(&std::env::args().next().unwrap());

        std::process::exit(0);
    }

    crate::application::Widget::run(()).unwrap();
}

fn usage(program: &str) {
    let path = std::path::Path::new(&program);

    println!("    {}", path.file_name().unwrap().to_str().unwrap());
    println!("      Launch focus graphical interface");
}
