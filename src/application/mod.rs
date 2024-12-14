mod globals;
mod preferences;

pub use globals::preferences::get as preferences;
pub use globals::tasks::get as tasks;

use globals::tasks::add as add_task;
use preferences::Preferences;

use gtk::glib::clone;
use gtk::prelude::*;
use relm4::ComponentController as _;

pub const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
enum Page {
    Inbox = 0,
    Projects,
    Contexts,
    Agenda,
    Flag,
    Done,
    Search,
}

impl From<u32> for Page {
    fn from(n: u32) -> Self {
        match n {
            0 => Page::Inbox,
            1 => Page::Projects,
            2 => Page::Contexts,
            3 => Page::Agenda,
            4 => Page::Flag,
            5 => Page::Done,
            6 => Page::Search,
            _ => panic!("Invalid page {n}"),
        }
    }
}

impl From<Page> for u32 {
    fn from(page: Page) -> u32 {
        unsafe { std::mem::transmute(page) }
    }
}

#[derive(Debug)]
pub enum Msg {
    Add(String),
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    EditCancel,
    EditDone(Box<crate::tasks::Task>),
    Refresh,
    Search(String),
}

pub struct Model {
    add_popover: gtk::Popover,
    agenda: relm4::Controller<crate::agenda::Model>,
    contexts: relm4::Controller<crate::widgets::tags::Model>,
    defered_button: gtk::CheckButton,
    done_button: gtk::CheckButton,
    done: relm4::Controller<crate::done::Model>,
    edit: relm4::Controller<crate::edit::Model>,
    flag: relm4::Controller<crate::flag::Model>,
    inbox: relm4::Controller<crate::inbox::Model>,
    logger: relm4::Controller<crate::logger::Model>,
    notebook: gtk::Notebook,
    projects: relm4::Controller<crate::widgets::tags::Model>,
    search: relm4::Controller<crate::search::Model>,
    #[allow(dead_code)]
    xdg: xdg::BaseDirectories,
}

impl Model {
    fn load_style(&self) {
        let css = gtk::CssProvider::new();
        if let Some(stylesheet) = self.stylesheet() {
            css.load_from_path(stylesheet);

            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().unwrap(),
                &css,
                0,
            );
        } else {
            log::error!("Unable to find stylesheet");
        }
    }

    fn stylesheet(&self) -> Option<std::path::PathBuf> {
        let mut stylesheet = "style_light.css";

        if let Ok(theme) = std::env::var("GTK_THEME") {
            if theme.ends_with(":dark") {
                stylesheet = "style_dark.css";
            }
        } else if let Some(setting) = gtk::Settings::default() {
            if setting.is_gtk_application_prefer_dark_theme() {
                stylesheet = "style_dark.css";
            }
        }

        self.find_data_file(stylesheet)
    }

    fn add_tab_widgets(&self, notebook: &gtk::Notebook) {
        let n = notebook.n_pages();

        for x in 0..n {
            let page = notebook.nth_page(Some(x)).unwrap();
            let widget = self.tab_widget(x);

            notebook.set_tab_label(&page, Some(&widget));
        }
    }

    fn tab_widget(&self, n: u32) -> gtk::Box {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.set_homogeneous(false);

        let title = match n.into() {
            Page::Inbox => "inbox",
            Page::Projects => "projects",
            Page::Contexts => "contexts",
            Page::Agenda => "agenda",
            Page::Flag => "flag",
            Page::Done => "done",
            Page::Search => "search",
        };

        if let Some(filename) = self.find_data_file(&format!("{title}.png")) {
            let image = gtk::Image::from_file(filename);
            image.set_icon_size(gtk::IconSize::Large);
            vbox.append(&image);
        } else {
            log::error!("Unable to find resource '{title}.png'");
        }

        let label = gtk::Label::new(Some(title));
        vbox.append(&label);

        vbox
    }

    #[cfg(not(debug_assertions))]
    fn find_data_file(&self, stylesheet: &str) -> Option<std::path::PathBuf> {
        self.xdg.find_data_file(stylesheet)
    }

    #[cfg(debug_assertions)]
    #[allow(clippy::unnecessary_wraps)]
    fn find_data_file(&self, stylesheet: &str) -> Option<std::path::PathBuf> {
        let mut path = std::path::PathBuf::new();

        path.push("resources");
        path.push(stylesheet);

        Some(path)
    }

    fn add(&mut self, text: &str) {
        match add_task(text) {
            Ok(_) => self.update_tasks(),
            Err(err) => log::error!("Unable to create task: '{err}'"),
        }

        self.add_popover.popdown();
    }

    fn complete(&mut self, task: &crate::tasks::Task) {
        let id = task.id;
        let mut list = tasks();

        if let Some(ref mut t) = list.tasks.get_mut(id) {
            if t.finished {
                t.uncomplete();
            } else {
                t.complete();
            }
        } else {
            return;
        }

        let t = list.tasks[id].clone();

        if t.finished {
            if let Some(ref recurrence) = t.recurrence {
                let due = if recurrence.strict && t.due_date.is_some() {
                    t.due_date.unwrap()
                } else {
                    crate::date::today()
                };

                let mut new: crate::tasks::Task = t.clone();
                new.uncomplete();
                new.create_date = Some(crate::date::today());
                new.due_date = Some(recurrence.clone() + due);

                if let Some(threshold_date) = t.threshold_date {
                    new.threshold_date = Some(recurrence.clone() + threshold_date);
                }

                list.append(new);
            }
        }

        match list.write() {
            Ok(_) => {
                if list.tasks[id].finished {
                    log::info!("Task done");
                } else {
                    log::info!("Task undone");
                }
            }
            Err(err) => log::error!("Unable to save tasks: {err}"),
        };

        self.update_tasks();
    }

    fn edit(&mut self, task: &crate::tasks::Task) {
        self.edit
            .emit(crate::edit::MsgInput::Set(Box::new(task.clone())));
        self.edit.widget().set_visible(true);
    }

    fn save(&mut self, task: &crate::tasks::Task) {
        let id = task.id;
        let mut list = tasks();

        if list.tasks.get_mut(id).is_some() {
            list.tasks[id] = task.clone();
        }

        match list.write() {
            Ok(_) => (),
            Err(err) => log::error!("Unable to save tasks: {err}"),
        };

        log::info!("Task updated");

        self.update_tasks();
        self.edit.widget().set_visible(false);
    }

    fn search(&self, query: &str) {
        if query.is_empty() {
            self.notebook.set_current_page(Some(Page::Inbox.into()));
            self.search.widget().set_visible(false);
        } else {
            self.search.widget().set_visible(true);
            self.notebook.set_current_page(Some(Page::Search.into()));
        }

        self.search
            .emit(crate::search::MsgInput::UpdateFilter(query.to_string()));
    }

    fn update_tasks(&self) {
        let todo_file = match std::env::var("TODO_FILE") {
            Ok(todo_file) => todo_file,
            Err(err) => {
                eprintln!("Launch this program via todo.sh: {err}");
                std::process::exit(1);
            }
        };

        let done_file = match std::env::var("DONE_FILE") {
            Ok(done_file) => done_file,
            Err(err) => {
                eprintln!("Launch this program via todo.sh: {err}");
                std::process::exit(1);
            }
        };

        let list = crate::tasks::List::from_files(&todo_file, &done_file);
        globals::tasks::replace(list);

        globals::preferences::replace(crate::application::Preferences {
            defered: self.defered_button.is_active(),
            done: self.done_button.is_active(),
        });

        self.agenda.sender().emit(crate::agenda::MsgInput::Update);
        self.contexts
            .sender()
            .emit(crate::widgets::tags::MsgInput::Update);
        self.done.sender().emit(crate::done::Msg::Update);
        self.projects
            .sender()
            .emit(crate::widgets::tags::MsgInput::Update);
        self.flag.sender().emit(crate::flag::Msg::Update);
        self.inbox.sender().emit(crate::inbox::Msg::Update);
        self.search.sender().emit(crate::search::MsgInput::Update);
    }

    fn watch(&self, sender: relm4::ComponentSender<Self>) {
        use notify::Watcher as _;

        let todo_dir = match std::env::var("TODO_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(err) => {
                eprintln!("Launch this program via todo.sh: {err}");
                std::process::exit(1);
            }
        };

        let mut watcher = notify::recommended_watcher(move |res| match res {
            Ok(_) => {
                sender.input(Msg::Refresh);
                log::info!("Tasks reloaded");
            }
            Err(e) => log::warn!("watch error: {e:?}"),
        })
        .unwrap();

        log::debug!("watching {todo_dir} for changes");

        if let Err(err) = watcher.watch(
            std::path::PathBuf::from(todo_dir).as_path(),
            notify::RecursiveMode::Recursive,
        ) {
            log::warn!("Unable to setup hot reload: {err}");
        }
    }
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Model {
    type Init = ();
    type Input = Msg;
    type Output = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        use relm4::Component as _;

        let logger = crate::logger::Model::builder().launch(()).detach();

        let agenda = crate::agenda::Model::builder()
            .launch(crate::date::today())
            .forward(sender.input_sender(), |output| match output {
                crate::agenda::MsgOutput::Complete(task) => Msg::Complete(task),
                crate::agenda::MsgOutput::Edit(task) => Msg::Edit(task),
            });

        let contexts = crate::widgets::tags::Model::builder()
            .launch(crate::widgets::tags::Type::Contexts)
            .forward(sender.input_sender(), |output| match output {
                crate::widgets::tags::MsgOutput::Complete(task) => Msg::Complete(task),
                crate::widgets::tags::MsgOutput::Edit(task) => Msg::Edit(task),
            });

        let done =
            crate::done::Model::builder()
                .launch(())
                .forward(sender.input_sender(), |output| match output {
                    crate::widgets::task::MsgOutput::Complete(task) => Msg::Complete(task),
                    crate::widgets::task::MsgOutput::Edit(task) => Msg::Edit(task),
                });

        let edit = crate::edit::Model::builder()
            .launch(crate::tasks::Task::new())
            .forward(sender.input_sender(), |output| match output {
                crate::edit::MsgOutput::Cancel => Msg::EditCancel,
                crate::edit::MsgOutput::Done(task) => Msg::EditDone(task),
            });

        let flag =
            crate::flag::Model::builder()
                .launch(())
                .forward(sender.input_sender(), |output| match output {
                    crate::widgets::task::MsgOutput::Complete(task) => Msg::Complete(task),
                    crate::widgets::task::MsgOutput::Edit(task) => Msg::Edit(task),
                });

        let inbox =
            crate::inbox::Model::builder()
                .launch(())
                .forward(sender.input_sender(), |output| match output {
                    crate::widgets::task::MsgOutput::Complete(task) => Msg::Complete(task),
                    crate::widgets::task::MsgOutput::Edit(task) => Msg::Edit(task),
                });

        let projects = crate::widgets::tags::Model::builder()
            .launch(crate::widgets::tags::Type::Projects)
            .forward(sender.input_sender(), |output| match output {
                crate::widgets::tags::MsgOutput::Complete(task) => Msg::Complete(task),
                crate::widgets::tags::MsgOutput::Edit(task) => Msg::Edit(task),
            });

        let search = crate::search::Model::builder()
            .launch(String::new())
            .forward(sender.input_sender(), |output| match output {
                crate::widgets::task::MsgOutput::Complete(task) => Msg::Complete(task),
                crate::widgets::task::MsgOutput::Edit(task) => Msg::Edit(task),
            });

        let defered_button = gtk::CheckButton::with_label("Display defered tasks");
        defered_button.connect_toggled(clone!(
            #[strong]
            sender,
            move |_| sender.input(Msg::Refresh)
        ));

        let done_button = gtk::CheckButton::with_label("Display done tasks");
        done_button.connect_toggled(clone!(
            #[strong]
            sender,
            move |_| sender.input(Msg::Refresh)
        ));

        let model = Self {
            add_popover: gtk::Popover::new(),
            agenda,
            contexts,
            defered_button,
            done_button,
            done,
            edit,
            flag,
            inbox,
            logger,
            notebook: gtk::Notebook::new(),
            projects,
            search,
            xdg: xdg::BaseDirectories::with_prefix(NAME.to_lowercase()).unwrap(),
        };

        let add_popover = &model.add_popover;
        let notebook = &model.notebook;
        let widgets = view_output!();

        model.load_style();
        model.add_tab_widgets(notebook);
        model.update_tasks();
        model.search.widget().set_visible(false);
        model.watch(sender);

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match msg {
            Msg::Add(task) => self.add(&task),
            Msg::Complete(task) => self.complete(&task),
            Msg::EditCancel => self.edit.widget().set_visible(false),
            Msg::EditDone(task) => self.save(&task),
            Msg::Edit(task) => self.edit(&task),
            Msg::Refresh => self.update_tasks(),
            Msg::Search(query) => self.search(&query),
        }
    }

    view! {
        gtk::ApplicationWindow {
            set_title: NAME.into(),
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                gtk::HeaderBar {
                    set_title_widget: Some(&gtk::Label::new(NAME.into())),

                    pack_start = &gtk::Button {
                        set_icon_name: "view-refresh",
                        set_tooltip_text: "Refresh".into(),

                        connect_clicked => Msg::Refresh,
                    },
                    pack_start = &gtk::MenuButton {
                        set_icon_name: "list-add",
                        set_tooltip_text: "Add".into(),
                        #[wrap(Some)]
                        #[local_ref]
                        set_popover = add_popover -> gtk::Popover {
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,

                                gtk::Entry {
                                    connect_activate[sender] => move |this| {
                                        sender.input(Msg::Add(this.text().to_string()));
                                    }
                                },
                                gtk::Label {
                                    set_text: "Create a new task +project @context due:2042-01-01",
                                },
                            },
                        },
                    },
                    pack_start = &gtk::MenuButton {
                        set_icon_name: "preferences-system",
                        set_tooltip_text: "Preferences".into(),
                        #[wrap(Some)]
                        set_popover = &gtk::Popover {
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                append: &model.defered_button,
                                append: &model.done_button,
                            },
                        },
                    },

                    pack_end = model.logger.widget(),
                    pack_end = &gtk::SearchEntry {
                        connect_search_changed[sender] => move |this| {
                            sender.input(Msg::Search(this.text().to_string()));
                        },
                    },
                },
                gtk::Paned {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_orientation: gtk::Orientation::Horizontal,
                    set_wide_handle: true,

                    #[wrap(Some)]
                    #[local_ref]
                    set_start_child = notebook -> gtk::Notebook {
                        set_tab_pos: gtk::PositionType::Left,

                        append_page: (model.inbox.widget(), None::<&gtk::Label>),
                        append_page: (model.projects.widget(), None::<&gtk::Label>),
                        append_page: (model.contexts.widget(), None::<&gtk::Label>),
                        append_page: (model.agenda.widget(), None::<&gtk::Label>),
                        append_page: (model.flag.widget(), None::<&gtk::Label>),
                        append_page: (model.done.widget(), None::<&gtk::Label>),
                        append_page: (model.search.widget(), None::<&gtk::Label>),
                    },
                    #[wrap(Some)]
                    set_end_child = model.edit.widget(),
                },
            },
            connect_close_request => move |_| {
                relm4::main_application().quit();
                gtk::glib::Propagation::Stop
            },
        }
    }
}
