use gtk::{
    self,
    NotebookExt,
    WidgetExt,
};
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl ::relm::Widget for Widget
{
    fn model(tasks: ::tasks::List) -> ::tasks::List
    {
        tasks
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
            gtk::Notebook {
                tab_pos: gtk::PositionType::Left,
                ::inbox::Widget(self.model.clone()) {
                    tab: {
                        tab_label: Some("Inbox"),
                    },
                },
            },
            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }
    }
}
