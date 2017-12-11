use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

use agenda::Msg::Complete as AgendaComplete;
use done::Msg::Complete as DoneComplete;
use inbox::Msg::Complete as InboxComplete;
use widgets::tags::Msg::Complete as TagsComplete;

pub struct Model {
    relm: ::relm::Relm<Widget>,
    list: ::tasks::List,
    popover: ::gtk::Popover,
    entry: ::gtk::Entry,
}

#[derive(Msg)]
pub enum Msg {
    Add,
    Create(Option<String>),
    Complete(::tasks::Task),
    Refresh,
    Quit,
}

impl Widget
{
    fn load_style(&self)
    {
        let screen = self.window.get_screen()
            .unwrap();
        let css = ::gtk::CssProvider::new();
        css.load_from_path("resources/style.css")
            .unwrap_or(error!("Invalid CSS"));

        ::gtk::StyleContext::add_provider_for_screen(&screen, &css, 0);
    }

    fn create_popover(&self)
    {
        let vbox = ::gtk::Box::new(::gtk::Orientation::Vertical, 0);

        vbox.add(&self.model.entry);
        self.model.entry.set_size_request(500, -1);
        connect!(self.model.relm, self.model.entry, connect_activate(entry), Msg::Create(entry.get_text()));

        let label = ::gtk::Label::new("Create a new task +project @context due:2042-01-01");
        vbox.add(&label);

        vbox.show_all();

        self.model.popover.set_relative_to(&self.add_button);
        self.model.popover.add(&vbox);
        self.model.popover.hide();
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

    fn add(&self)
    {
        self.model.entry.set_text("");
        self.model.popover.popup();
    }

    fn create(&mut self, text: Option<String>)
    {
        if let Some(text) = text {
            match self.model.list.add(&text) {
                Ok(_) => self.update_tasks(),
                Err(err) => error!("Unable to create task: '{}'", err),
            }
        }
        self.model.popover.popdown();
    }

    fn complete(&mut self, task: &::tasks::Task)
    {
        let id = task.id;
        let mut list = self.model.list.clone();

        if let Some(ref mut t) = list.tasks.get_mut(id) {
            if !t.finished {
                t.complete();
            }
            else {
                t.uncomplete();
            }
        }

        match list.write() {
            Ok(_) => (),
            Err(err) => error!("Unable to save tasks: {}", err),
        };

        self.update_tasks();
    }

    fn update_tasks(&mut self)
    {
        let todo_file = match ::std::env::var("TODO_FILE") {
            Ok(todo_file) => todo_file,
            Err(_) => panic!("Launch this program via todo.sh"),
        };

        let done_file = match ::std::env::var("DONE_FILE") {
            Ok(done_file) => done_file,
            Err(_) => panic!("Launch this program via todo.sh"),
        };

        let list = ::tasks::List::from_files(&todo_file, &done_file);

        self.inbox.emit(::inbox::Msg::Update(list.clone()));
        self.projects.emit(::widgets::tags::Msg::Update(list.clone()));
        self.contexts.emit(::widgets::tags::Msg::Update(list.clone()));
        self.agenda.emit(::agenda::Msg::Update(list.clone()));
        self.done.emit(::done::Msg::Update(list.clone()));

        self.model.list = list;
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.load_style();
        self.create_popover();
        self.replace_tab_widgets();
        self.update_tasks();
    }

    fn model(relm: &::relm::Relm<Self>, _: ()) -> Model
    {
        Model {
            relm: relm.clone(),
            list: ::tasks::List::new(),
            popover: ::gtk::Popover::new(None::<&::gtk::Button>),
            entry: ::gtk::Entry::new(),
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Add => self.add(),
            Create(text) => self.create(text),
            Complete(task) => self.complete(&task),
            Refresh => self.update_tasks(),
            Quit => ::gtk::main_quit(),
        }
    }

    view!
    {
        #[name="window"]
        gtk::Window {
            gtk::Box {
                orientation: ::gtk::Orientation::Vertical,
                gtk::Toolbar {
                    style: ::gtk::ToolbarStyle::Both,
                    gtk::ToolButton {
                        icon_name: "view-refresh",
                        label: "Refresh",
                        clicked => Msg::Refresh,
                    },
                    #[name="add_button"]
                    gtk::ToolButton {
                        icon_name: "list-add",
                        label: "Add",
                        clicked => Msg::Add,
                    },
                },
                #[name="notebook"]
                gtk::Notebook {
                    packing: {
                        expand: true,
                        fill: true,
                    },
                    tab_pos: ::gtk::PositionType::Left,
                    #[name="inbox"]
                    ::inbox::Widget {
                        InboxComplete(ref task) => Msg::Complete(task.clone()),
                    },
                    #[name="projects"]
                    ::widgets::Tags(::widgets::tags::Type::Projects) {
                        TagsComplete(ref task) => Msg::Complete(task.clone()),
                    },
                    #[name="contexts"]
                    ::widgets::Tags(::widgets::tags::Type::Contexts) {
                        TagsComplete(ref task) => Msg::Complete(task.clone()),
                    },
                    #[name="agenda"]
                    ::agenda::Widget {
                        AgendaComplete(ref task) => Msg::Complete(task.clone()),
                    },
                    #[name="done"]
                    ::done::Widget {
                        DoneComplete(ref task) => Msg::Complete(task.clone()),
                    },
                },
            },
            delete_event(_, _) => (Msg::Quit, ::gtk::Inhibit(false)),
        }
    }
}
