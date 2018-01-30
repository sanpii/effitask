use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Add(Option<String>),
}

#[widget]
impl ::relm::Widget for Widget
{
    fn model(_: ()) -> ()
    {
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Add(_) => self.entry.set_text(""),
        }
    }

    view!
    {
        gtk::Box {
            orientation: ::gtk::Orientation::Vertical,
            #[name="entry"]
            gtk::Entry {
                activate(entry) => Msg::Add(entry.get_text()),
            },
            gtk::Label {
                text: "Create a new task +project @context due:2042-01-01",
            },
        }
    }
}
