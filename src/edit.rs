use crate::widgets::calendar::Msg::Updated as CalendarUpdated;
use crate::widgets::keywords::Msg::Updated as KeywordsUpdated;
use crate::widgets::priority::Msg::Updated as PriorityUpdated;
use crate::widgets::repeat::Msg::Updated as RepeatUpdated;
use crate::widgets::{Calendar, Keywords, Priority, Repeat};
use gtk::prelude::*;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Cancel,
    EditKeyword(std::collections::BTreeMap<String, String>),
    Flag,
    Done(Box<crate::tasks::Task>),
    Ok,
    Set(Box<crate::tasks::Task>),
    UpdateDate(DateType, Option<chrono::NaiveDate>),
    UpdateRepeat(Option<todo_txt::task::Recurrence>),
    UpdatePriority(u8),
}

pub struct Model {
    task: crate::tasks::Task,
    relm: relm::Relm<Widget>,
}

#[derive(Clone, Copy)]
pub enum DateType {
    Due,
    Threshold,
    Finish,
}

impl Widget {
    fn set_task(&mut self, task: &crate::tasks::Task) {
        self.model.task = task.clone();

        self.widgets.subject.set_text(task.subject.as_str());
        self.components
            .priority
            .emit(crate::widgets::priority::Msg::Set(
                task.priority.clone().into(),
            ));
        self.widgets.flag.set_active(task.flagged);
        self.components
            .due
            .emit(crate::widgets::calendar::Msg::Set(task.due_date));
        self.components
            .threshold
            .emit(crate::widgets::calendar::Msg::Set(task.threshold_date));
        if task.create_date.is_some() {
            self.components
                .created
                .emit(crate::widgets::calendar::Msg::Set(task.create_date));
            self.widgets.created.show();
        } else {
            self.widgets.created.hide();
        }
        self.components
            .repeat
            .emit(crate::widgets::repeat::Msg::Set(task.recurrence.clone()));
        self.components
            .finish
            .emit(crate::widgets::calendar::Msg::Set(task.finish_date));
        self.components
            .keywords
            .emit(crate::widgets::keywords::Msg::Set(task.tags.clone()));

        let note = task.note.content().unwrap_or_default();
        let buffer = self.widgets.note.buffer().unwrap();
        buffer.set_text(note.as_str());
    }

    fn get_task(&self) -> crate::tasks::Task {
        let mut task = self.model.task.clone();

        task.subject = self.widgets.subject.text().to_string();

        let new_note = self.get_note();
        task.note = match task.note {
            todo_txt::task::Note::Long { ref filename, .. } => todo_txt::task::Note::Long {
                filename: filename.to_string(),
                content: new_note,
            },
            _ => {
                if new_note.is_empty() {
                    todo_txt::task::Note::None
                } else {
                    todo_txt::task::Note::Short(new_note)
                }
            }
        };

        task
    }

    fn get_note(&self) -> String {
        let Some(buffer) = self.widgets.note.buffer() else {
            return String::new();
        };
        let start = buffer.start_iter();
        let end = buffer.end_iter();

        buffer.text(&start, &end, false).expect("").to_string()
    }

    fn edit_keywords(&mut self, keywords: &std::collections::BTreeMap<String, String>) {
        self.model.task.tags = keywords.clone();
    }

    fn flag(&mut self) {
        self.model.task.flagged = self.widgets.flag.is_active();
    }

    fn update_date(&mut self, date_type: DateType, date: Option<chrono::NaiveDate>) {
        use DateType::*;

        match date_type {
            Due => self.model.task.due_date = date,
            Threshold => self.model.task.threshold_date = date,
            Finish => {
                self.model.task.finish_date = date;
                self.model.task.finished = date.is_some();
            }
        }
    }

    fn update_repeat(&mut self, recurrence: &Option<todo_txt::task::Recurrence>) {
        self.model.task.recurrence.clone_from(recurrence);
    }

    fn update_priority(&mut self, priority: u8) {
        self.model.task.priority = priority.into();
    }
}

#[allow(clippy::cognitive_complexity)]
#[relm_derive::widget]
impl relm::Widget for Widget {
    fn init_view(&mut self) {
        self.widgets.note.set_height_request(150);
        self.widgets.created.set_sensitive(false);
    }

    fn model(relm: &relm::Relm<Self>, _: ()) -> Model {
        Model {
            task: crate::tasks::Task::new(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Cancel | Done(_) => (),
            EditKeyword(ref keywords) => self.edit_keywords(keywords),
            Flag => self.flag(),
            Ok => self
                .model
                .relm
                .stream()
                .emit(Msg::Done(Box::new(self.get_task()))),
            Set(task) => self.set_task(&task),
            UpdateDate(ref date_type, ref date) => self.update_date(*date_type, *date),
            UpdateRepeat(ref recurrence) => self.update_repeat(recurrence),
            UpdatePriority(priority) => self.update_priority(priority),
        }
    }

    view! {
        gtk::ScrolledWindow {
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                spacing: 10,
                gtk::Frame {
                    label: Some("Subject"),
                    #[name="subject"]
                    gtk::Entry {
                        activate => Msg::Ok,
                    },
                },
                gtk::Frame {
                    label: Some("Priority"),
                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        #[name="priority"]
                        Priority {
                            PriorityUpdated(priority) => Msg::UpdatePriority(priority),
                        },
                        #[name="flag"]
                        gtk::ToggleButton {
                            child: {
                                expand: true,
                            },
                            halign: gtk::Align::Center,
                            image: Some(&gtk::Image::from_icon_name(Some("emblem-favorite"), gtk::IconSize::SmallToolbar)),
                            tooltip_text: Some("Flag"),
                            toggled => Msg::Flag,
                        },
                    },
                },
                gtk::Frame {
                    label: Some("Date"),
                    gtk::Box {
                        spacing: 10,
                        orientation: gtk::Orientation::Vertical,
                        #[name="threshold"]
                        Calendar("Defer until".to_string()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Threshold, date),
                        },
                        #[name="due"]
                        Calendar("Due".to_string()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Due, date),
                        },
                        #[name="finish"]
                        Calendar("Completed".to_string()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Finish, date),
                        },
                        #[name="created"]
                        Calendar("Created".to_string()),
                    },
                },
                gtk::Frame {
                    label: Some("Repeat"),
                    #[name="repeat"]
                    Repeat {
                        RepeatUpdated(ref recurrence) => Msg::UpdateRepeat(recurrence.clone()),
                    },
                },
                gtk::Frame {
                    label: Some("Keywords"),
                    #[name="keywords"]
                    Keywords {
                        KeywordsUpdated(ref keywords) => Msg::EditKeyword(keywords.clone()),
                    },
                },
                gtk::Frame {
                    label: Some("Note"),
                    #[name="note"]
                    gtk::TextView {
                    },
                },
                gtk::ActionBar {
                    child: {
                        pack_type: gtk::PackType::End,
                    },
                    gtk::ButtonBox {
                        orientation: gtk::Orientation::Horizontal,
                        gtk::Button {
                            label: "Ok",
                            clicked => Msg::Ok,
                        },
                        gtk::Button {
                            label: "Cancel",
                            clicked => Msg::Cancel,
                        },
                    },
                },
            },
        }
    }
}
