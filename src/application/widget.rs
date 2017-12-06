use gtk;
use gtk::prelude::*;
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
        css.load_from_data(b"
treeview {
    font-size: 20px;
}

expander {
    font-size: 30px;
}

arrow {
    min-width: 32px;
    min-height: 32px;
}
").unwrap_or(error!("Invalid CSS"));

        ::gtk::StyleContext::add_provider_for_screen(&screen, &css, 0);
    }

    fn replace_tab_widgets(&self)
    {
        let n = self.notebook.get_n_pages();

        for x in 0..n {
            let page = self.notebook.get_nth_page(Some(x))
                .unwrap();
            let widget = self.get_tab_widget(x);

            self.notebook.set_tab_label(&page, Some(&widget));
        }
    }

    fn get_tab_widget(&self, n: u32) -> ::gtk::Box
    {
        let vbox = ::gtk::Box::new(::gtk::Orientation::Vertical, 0);
        let title = match n {
            0 => "inbox",
            1 => "projects",
            2 => "contexts",
            3 => "agenda",
            4 => "done",
            _ => {
                error!("Invalid tab n°{}", n);

                ""
            },
        };
        let image = ::gtk::Image::new_from_file(format!("resources/{}.png", title));
        vbox.pack_start(&image, false, false, 0);

        let label = ::gtk::Label::new(Some(title));
        vbox.pack_start(&label, false, false, 0);

        vbox.show_all();

        vbox
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.load_style();
        self.replace_tab_widgets();
    }

    fn model(tasks: ::tasks::List) -> ::tasks::List
    {
        tasks
    }

    fn update(&mut self, event: Msg)
    {
        match event {
            Msg::Quit => ::gtk::main_quit(),
        }
    }

    view!
    {
        #[name="window"]
        gtk::Window {
            #[name="notebook"]
            gtk::Notebook {
                tab_pos: ::gtk::PositionType::Left,
                ::inbox::Widget(self.model.clone()),
                ::projects::Widget(self.model.clone()),
                ::contexts::Widget(self.model.clone()),
                ::agenda::Widget(self.model.clone()),
                ::done::Widget(self.model.clone()),
            },
            delete_event(_, _) => (Msg::Quit, ::gtk::Inhibit(false)),
        }
    }
}
