use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

use agenda::Msg::Complete as AgendaComplete;
use agenda::Msg::Edit as AgendaEdit;
use done::Msg::Complete as DoneComplete;
use done::Msg::Edit as DoneEdit;
use edit::Msg::{Cancel, Done};
use inbox::Msg::Complete as InboxComplete;
use inbox::Msg::Edit as InboxEdit;
use widgets::tags::Msg::Complete as TagsComplete;
use widgets::tags::Msg::Edit as TagsEdit;

const NAME: &str = "Effitask";

pub struct Model {
    relm: ::relm::Relm<Widget>,
    list: ::tasks::List,
    popover: ::gtk::Popover,
    entry: ::gtk::Entry,
    xdg: ::xdg::BaseDirectories,
}

#[derive(Msg)]
pub enum Msg {
    Add,
    Create(Option<String>),
    Complete(::tasks::Task),
    Edit(::tasks::Task),
    EditCancel,
    EditDone(::tasks::Task),
    Refresh,
    Show,
    SwitchPage,
    Quit,
}

impl Widget
{
    fn load_style(&self)
    {
        let screen = self.window.get_screen()
            .unwrap();
        let css = ::gtk::CssProvider::new();
        if let Some(stylesheet) = self.get_stylesheet() {
            css.load_from_path(stylesheet.to_str().unwrap())
                .unwrap_or(error!("Invalid CSS"));

            ::gtk::StyleContext::add_provider_for_screen(&screen, &css, 0);
        }
        else {
            error!("Unable to find stylesheet");
        }
    }

    fn get_stylesheet(&self) -> Option<::std::path::PathBuf>
    {
        let mut stylesheet = "style_light.css";

        if let Some(setting) = ::gtk::Settings::get_default() {
            if setting.get_property_gtk_application_prefer_dark_theme() {
                stylesheet = "style_dark.css";
            }
        }

        self.model.xdg.find_data_file(stylesheet)
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

        if let Some(filename) = self.model.xdg.find_data_file(format!("{}.png", title).as_str()) {
            let image = ::gtk::Image::new_from_file(filename);
            vbox.pack_start(&image, false, false, 0);
        }
        else {
            error!("Unable to find resource '{}.png'", title);
        }

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

    fn edit(&mut self, task: &::tasks::Task)
    {
        self.edit.emit(::edit::Msg::Set(task.clone()));
        self.edit.widget()
            .show();
    }

    fn save(&mut self, task: &::tasks::Task)
    {
        let id = task.id;
        let mut list = self.model.list.clone();

        if list.tasks.get_mut(id).is_some() {
            ::std::mem::replace(&mut list.tasks[id], task.clone());
        }

        match list.write() {
            Ok(_) => (),
            Err(err) => error!("Unable to save tasks: {}", err),
        };

        self.update_tasks();
        self.edit.widget().hide();
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

    fn on_show(&self)
    {
        let width = self.window.get_allocated_width();
        self.paned.set_position(width - 400);
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.edit.widget()
            .hide();

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
            xdg: ::xdg::BaseDirectories::with_prefix(::application::NAME.to_lowercase())
                .unwrap(),
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Add => self.add(),
            Create(text) => self.create(text),
            Complete(task) => self.complete(&task),
            Edit(task) => self.edit(&task),
            EditDone(task) => self.save(&task),
            EditCancel => self.edit.widget().hide(),
            Refresh => self.update_tasks(),
            Show => self.on_show(),
            SwitchPage => self.edit.widget().hide(),
            Quit => ::gtk::main_quit(),
        }
    }

    view!
    {
        #[name="window"]
        gtk::Window {
            title: NAME,
            gtk::Box {
                orientation: ::gtk::Orientation::Vertical,
                gtk::HeaderBar {
                    title: NAME,
                    show_close_button: true,
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
                #[name="paned"]
                gtk::Paned {
                    packing: {
                        expand: true,
                        fill: true,
                    },
                    orientation: ::gtk::Orientation::Horizontal,
                    wide_handle: true,
                    #[name="notebook"]
                    gtk::Notebook {
                        tab_pos: ::gtk::PositionType::Left,
                        #[name="inbox"]
                        ::inbox::Widget {
                            InboxComplete(ref task) => Msg::Complete(task.clone()),
                            InboxEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="projects"]
                        ::widgets::Tags(::widgets::tags::Type::Projects) {
                            TagsComplete(ref task) => Msg::Complete(task.clone()),
                            TagsEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="contexts"]
                        ::widgets::Tags(::widgets::tags::Type::Contexts) {
                            TagsComplete(ref task) => Msg::Complete(task.clone()),
                            TagsEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="agenda"]
                        ::agenda::Widget {
                            AgendaComplete(ref task) => Msg::Complete(task.clone()),
                            AgendaEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="done"]
                        ::done::Widget {
                            DoneComplete(ref task) => Msg::Complete(task.clone()),
                            DoneEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        switch_page(_, _, _) => Msg::SwitchPage,
                    },
                    #[name="edit"]
                    ::edit::Widget {
                        Cancel => Msg::EditCancel,
                        Done(ref task) => Msg::EditDone(task.clone()),
                    },
                },
            },
            size_allocate(_, _) => Msg::Show,
            delete_event(_, _) => (Msg::Quit, ::gtk::Inhibit(false)),
        }
    }
}
