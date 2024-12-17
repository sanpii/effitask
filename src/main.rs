#![warn(warnings)]

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
    human_panic::setup_panic!();

    envir::init();

    if std::env::args().nth(1).as_deref() == Some("usage") {
        usage(&std::env::args().next().unwrap());

        std::process::exit(0);
    }

    let app = relm4::RelmApp::new("txt.todo.effitask")
        .with_args(Vec::new());
    app.run::<application::Model>(());
}

fn usage(program: &str) {
    let path = std::path::Path::new(&program);

    println!("    {}", path.file_name().unwrap().to_str().unwrap());
    println!("      Launch focus graphical interface");
}
