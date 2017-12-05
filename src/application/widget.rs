use gtk::{
    self,
    CssProviderExt,
    NotebookExt,
    WidgetExt,
};
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

impl Widget
{
    fn load_style(&self)
    {
        let screen = self.window.get_screen()
            .unwrap();
        let css = ::gtk::CssProvider::new();
        css.load_from_data(b"treeview { font-size: 20px; }")
            .unwrap_or(error!("Invalid CSS"));

        ::gtk::StyleContext::add_provider_for_screen(&screen, &css, 0);
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.load_style();
    }

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
        #[name="window"]
        gtk::Window {
            gtk::Notebook {
                tab_pos: gtk::PositionType::Left,
                ::inbox::Widget(self.model.clone()) {
                    tab: {
                        tab_label: Some("Inbox"),
                    },
                },
                ::projects::Widget(self.model.clone()) {
                    tab: {
                        tab_label: Some("Projects"),
                    },
                },
                ::contexts::Widget(self.model.clone()) {
                    tab: {
                        tab_label: Some("Contexts"),
                    },
                },
                ::done::Widget(self.model.clone()) {
                    tab: {
                        tab_label: Some("Done"),
                    },
                },
            },
            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }
    }
}
