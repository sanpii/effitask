use gtk::prelude::*;

use crate::agenda::Msg::Complete as AgendaComplete;
use crate::agenda::Msg::Edit as AgendaEdit;
use crate::agenda::Widget as AgendaWidget;
use crate::done::Msg::Complete as DoneComplete;
use crate::done::Msg::Edit as DoneEdit;
use crate::done::Widget as DoneWidget;
use crate::edit::Msg::{Cancel, Done};
use crate::edit::Widget as EditWidget;
use crate::flag::Msg::Complete as FlagComplete;
use crate::flag::Msg::Edit as FlagEdit;
use crate::flag::Widget as FlagWidget;
use crate::inbox::Msg::Complete as InboxComplete;
use crate::inbox::Msg::Edit as InboxEdit;
use crate::inbox::Widget as InboxWidget;
use crate::logger::Widget as LoggerWidget;
use crate::search::Msg::Complete as SearchComplete;
use crate::search::Msg::Edit as SearchEdit;
use crate::search::Widget as SearchWidget;
use crate::widgets::tags::Msg::Complete as TagsComplete;
use crate::widgets::tags::Msg::Edit as TagsEdit;
use crate::widgets::Tags as TagsWidget;

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

impl std::convert::From<u32> for Page {
    fn from(n: u32) -> Self {
        match n {
            0 => Page::Inbox,
            1 => Page::Projects,
            2 => Page::Contexts,
            3 => Page::Agenda,
            4 => Page::Flag,
            5 => Page::Done,
            6 => Page::Search,
            _ => panic!("Invalid page {}", n),
        }
    }
}

impl From<Page> for i32 {
    fn from(page: Page) -> i32 {
        unsafe { std::mem::transmute(page) }
    }
}

pub struct Model {
    relm: relm::Relm<Widget>,
    add: relm::Component<crate::add::Widget>,
    add_popover: gtk::Popover,
    pref_popover: gtk::Popover,
    defered_button: gtk::CheckButton,
    done_button: gtk::CheckButton,
    #[allow(dead_code)]
    xdg: xdg::BaseDirectories,
}

#[derive(relm_derive::Msg)]
pub enum Msg {
    Add,
    Create(String),
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    EditCancel,
    EditDone(Box<crate::tasks::Task>),
    Preferences,
    Refresh,
    Search(String),
    SwitchPage,
}

impl Widget {
    fn load_style(&self) {
        let screen = self.widgets.window.screen().unwrap();
        let css = gtk::CssProvider::new();
        if let Some(stylesheet) = self.get_stylesheet() {
            match css.load_from_path(stylesheet.to_str().unwrap()) {
                Ok(_) => (),
                Err(err) => log::error!("Invalid CSS: {}", err),
            }

            gtk::StyleContext::add_provider_for_screen(&screen, &css, 0);
        } else {
            log::error!("Unable to find stylesheet");
        }
    }

    fn get_stylesheet(&self) -> Option<std::path::PathBuf> {
        let mut stylesheet = "style_light.css";

        if let Ok(theme) = std::env::var("GTK_THEME") {
            if theme.ends_with(":dark") {
                stylesheet = "style_dark.css";
            }
        } else if let Some(setting) = self.widgets.window.settings() {
            use gtk::traits::SettingsExt;

            if setting.is_gtk_application_prefer_dark_theme() {
                stylesheet = "style_dark.css";
            }
        }

        self.find_data_file(stylesheet)
    }

    #[cfg(not(debug_assertions))]
    fn find_data_file(&self, stylesheet: &str) -> Option<std::path::PathBuf> {
        self.model.xdg.find_data_file(stylesheet)
    }

    #[cfg(debug_assertions)]
    #[allow(clippy::unnecessary_wraps)]
    fn find_data_file(&self, stylesheet: &str) -> Option<std::path::PathBuf> {
        let mut path = std::path::PathBuf::new();

        path.push("resources");
        path.push(stylesheet);

        Some(path)
    }

    fn init_add_popover(&mut self) {
        let add = &self.model.add;

        relm::connect!(add@crate::add::Msg::Add(ref text), self.model.relm, Msg::Create(text.clone()));

        self.model
            .add_popover
            .set_relative_to(Some(&self.widgets.add_button));
        self.model.add_popover.hide();
    }

    fn init_pref_popover(&self) {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.show();

        relm::connect!(
            self.model.relm,
            self.model.defered_button,
            connect_toggled(_),
            Msg::Refresh
        );
        vbox.add(&self.model.defered_button);
        self.model.defered_button.show();

        relm::connect!(
            self.model.relm,
            self.model.done_button,
            connect_toggled(_),
            Msg::Refresh
        );
        vbox.add(&self.model.done_button);
        self.model.done_button.show();

        self.model
            .pref_popover
            .set_relative_to(Some(&self.widgets.pref_button));
        self.model.pref_popover.add(&vbox);
        self.model.pref_popover.hide();
    }

    fn replace_tab_widgets(&self) {
        let n = self.widgets.notebook.n_pages();

        for x in 0..n {
            let page = self.widgets.notebook.nth_page(Some(x)).unwrap();
            let widget = self.get_tab_widget(x);

            self.widgets.notebook.set_tab_label(&page, Some(&widget));
        }
    }

    fn get_tab_widget(&self, n: u32) -> gtk::Box {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let title = match n.into() {
            Page::Inbox => "inbox",
            Page::Projects => "projects",
            Page::Contexts => "contexts",
            Page::Agenda => "agenda",
            Page::Flag => "flag",
            Page::Done => "done",
            Page::Search => "search",
        };

        if let Some(filename) = self.find_data_file(format!("{}.png", title).as_str()) {
            let image = gtk::Image::from_file(filename);
            vbox.pack_start(&image, false, false, 0);
        } else {
            log::error!("Unable to find resource '{}.png'", title);
        }

        let label = gtk::Label::new(Some(title));
        vbox.pack_start(&label, false, false, 0);

        vbox.show_all();

        vbox
    }

    fn add(&self) {
        self.model.add_popover.popup();
    }

    fn create(&mut self, text: String) {
        match super::add_task(&text) {
            Ok(_) => self.update_tasks(),
            Err(err) => log::error!("Unable to create task: '{}'", err),
        }

        self.model.add_popover.popdown();
    }

    fn complete(&mut self, task: &crate::tasks::Task) {
        let id = task.id;
        let mut list = super::tasks();

        if let Some(ref mut t) = list.tasks.get_mut(id) {
            if !t.finished {
                t.complete();
            } else {
                t.uncomplete();
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
            Err(err) => log::error!("Unable to save tasks: {}", err),
        };

        self.update_tasks();
    }

    fn edit(&mut self, task: &crate::tasks::Task) {
        use relm::Widget;

        self.components
            .edit
            .emit(crate::edit::Msg::Set(Box::new(task.clone())));
        self.widgets.edit.show();

        let (width, _) = self.root().size();
        self.widgets.paned.set_position(width - 436);
    }

    fn save(&mut self, task: &crate::tasks::Task) {
        let id = task.id;
        let mut list = super::tasks();

        if list.tasks.get_mut(id).is_some() {
            list.tasks[id] = task.clone();
        }

        match list.write() {
            Ok(_) => (),
            Err(err) => log::error!("Unable to save tasks: {}", err),
        };

        log::info!("Task updated");

        self.update_tasks();
        self.widgets.edit.hide();
    }

    fn search(&self, text: &str) {
        if text.is_empty() {
            self.widgets.notebook.set_page(Page::Inbox.into());
            self.widgets.search.hide();
        } else {
            self.widgets.search.show();
            self.widgets.notebook.set_page(Page::Search.into());
        }

        self.components
            .search
            .emit(crate::search::Msg::UpdateFilter(text.to_string()));
    }

    fn update_tasks(&mut self) {
        let todo_file = match std::env::var("TODO_FILE") {
            Ok(todo_file) => todo_file,
            Err(err) => {
                eprintln!("Launch this program via todo.sh: {}", err);
                std::process::exit(1);
            }
        };

        let done_file = match std::env::var("DONE_FILE") {
            Ok(done_file) => done_file,
            Err(err) => {
                eprintln!("Launch this program via todo.sh: {}", err);
                std::process::exit(1);
            }
        };

        let list = crate::tasks::List::from_files(&todo_file, &done_file);
        super::globals::tasks::replace(list);

        super::globals::preferences::replace(crate::application::Preferences {
            defered: self.model.defered_button.is_active(),
            done: self.model.done_button.is_active(),
        });

        self.components.inbox.emit(crate::inbox::Msg::Update);
        self.components
            .projects
            .emit(crate::widgets::tags::Msg::Update);
        self.components
            .contexts
            .emit(crate::widgets::tags::Msg::Update);
        self.components.agenda.emit(crate::agenda::Msg::Update);
        self.components.done.emit(crate::done::Msg::Update);
        self.components.flag.emit(crate::flag::Msg::Update);
        self.components.search.emit(crate::search::Msg::Update);
    }

    fn preferences(&self) {
        self.model.pref_popover.popup();
    }

    fn watch(&self) {
        use notify::Watcher;

        let stream = self.model.relm.stream().clone();

        let (_, sender) = relm::Channel::new(move |_| {
            stream.emit(Msg::Refresh);
            log::info!("Tasks reloaded");
        });


        let todo_dir = match std::env::var("TODO_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(err) => {
                eprintln!("Launch this program via todo.sh: {}", err);
                std::process::exit(1);
            }
        };

        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(_) => {
                    sender.send(()).expect("send message");
                }
                Err(e) => log::warn!("watch error: {:?}", e),
            }
        }).unwrap();

        log::debug!("watching {} for changes", todo_dir);

        watcher
            .watch(std::path::PathBuf::from(todo_dir).as_path(), notify::RecursiveMode::Recursive)
            .unwrap();
    }
}

#[relm_derive::widget]
impl relm::Widget for Widget {
    fn init_view(&mut self) {
        self.widgets.edit.hide();
        self.widgets.search.hide();

        self.load_style();
        self.init_add_popover();
        self.init_pref_popover();
        self.replace_tab_widgets();
        self.update_tasks();

        self.watch();
    }

    fn model(relm: &relm::Relm<Self>, _: ()) -> Model {
        use relm::ContainerWidget;

        let add_popover = gtk::Popover::new(None::<&gtk::Button>);

        Model {
            relm: relm.clone(),
            add: add_popover.add_widget::<crate::add::Widget>(()),
            add_popover,
            pref_popover: gtk::Popover::new(None::<&gtk::Button>),
            defered_button: gtk::CheckButton::with_label("Display defered tasks"),
            done_button: gtk::CheckButton::with_label("Display done tasks"),
            xdg: xdg::BaseDirectories::with_prefix(super::NAME.to_lowercase()).unwrap(),
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Add => self.add(),
            Create(text) => self.create(text),
            Complete(task) => self.complete(&task),
            Edit(task) => self.edit(&task),
            EditDone(task) => self.save(&task),
            EditCancel => self.widgets.edit.hide(),
            Preferences => self.preferences(),
            Refresh => self.update_tasks(),
            Search(text) => self.search(&text),
            SwitchPage => self.widgets.edit.hide(),
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            title: super::NAME,
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                gtk::HeaderBar {
                    title: Some(super::NAME),
                    show_close_button: true,
                    gtk::ToolButton {
                        icon_name: Some("view-refresh"),
                        label: Some("Refresh"),
                        clicked => Msg::Refresh,
                    },
                    #[name="add_button"]
                    gtk::ToolButton {
                        icon_name: Some("list-add"),
                        label: Some("Add"),
                        clicked => Msg::Add,
                    },
                    #[name="pref_button"]
                    gtk::ToolButton {
                        icon_name: Some("preferences-system"),
                        label: Some("Preferences"),
                        clicked => Msg::Preferences,
                    },
                    gtk::SearchEntry {
                        child: {
                            pack_type: gtk::PackType::End,
                        },
                        search_changed(entry) => Msg::Search(entry.text().to_string()),
                    },
                    LoggerWidget {
                        child: {
                            pack_type: gtk::PackType::End,
                        },
                    },
                },
                #[name="paned"]
                gtk::Paned {
                    child: {
                        expand: true,
                        fill: true,
                    },
                    orientation: gtk::Orientation::Horizontal,
                    wide_handle: true,
                    #[name="notebook"]
                    gtk::Notebook {
                        tab_pos: gtk::PositionType::Left,
                        #[name="inbox"]
                        InboxWidget {
                            InboxComplete(ref task) => Msg::Complete(task.clone()),
                            InboxEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="projects"]
                        TagsWidget(crate::widgets::tags::Type::Projects) {
                            TagsComplete(ref task) => Msg::Complete(task.clone()),
                            TagsEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="contexts"]
                        TagsWidget(crate::widgets::tags::Type::Contexts) {
                            TagsComplete(ref task) => Msg::Complete(task.clone()),
                            TagsEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="agenda"]
                        AgendaWidget {
                            AgendaComplete(ref task) => Msg::Complete(task.clone()),
                            AgendaEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="flag"]
                        FlagWidget {
                            FlagComplete(ref task) => Msg::Complete(task.clone()),
                            FlagEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="done"]
                        DoneWidget {
                            DoneComplete(ref task) => Msg::Complete(task.clone()),
                            DoneEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        #[name="search"]
                        SearchWidget {
                            SearchComplete(ref task) => Msg::Complete(task.clone()),
                            SearchEdit(ref task) => Msg::Edit(task.clone()),
                        },
                        switch_page(_, _, _) => Msg::SwitchPage,
                    },
                    #[name="edit"]
                    EditWidget {
                        Cancel => Msg::EditCancel,
                        Done(ref task) => Msg::EditDone(task.clone()),
                    },
                },
            },
            delete_event(_, _) => (gtk::main_quit(), gtk::Inhibit(false)),
        }
    }
}
