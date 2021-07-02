use gtk::prelude::*;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Add(String),
}

#[relm_derive::widget]
impl relm::Widget for Widget {
    fn model(_: ()) {}

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Add(_) => self.widgets.entry.set_text(""),
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            #[name="entry"]
            gtk::Entry {
                activate(entry) => Msg::Add(entry.text().to_string()),
            },
            gtk::Label {
                text: "Create a new task +project @context due:2042-01-01",
            },
        }
    }
}
