use gtk::{self, WidgetExt};
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl ::relm::Widget for Widget
{
    fn model() -> ()
    {
        ()
    }

    fn update(&mut self, event: Msg)
    {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }

    view!
    {
        gtk::Window {
            delete_event(_, _) => (Msg::Quit, ::gtk::Inhibit(false)),
        }
    }
}
