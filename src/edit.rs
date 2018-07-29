use gtk;
use gtk::prelude::*;
use relm_attributes::widget;
use widgets::calendar::Msg::Updated as CalendarUpdated;
use widgets::keywords::Msg::Updated as KeywordsUpdated;
use widgets::priority::Msg::Updated as PriorityUpdated;
use widgets::repeat::Msg::Updated as RepeatUpdated;
use widgets::{Calendar, Keywords, Priority, Repeat};

#[derive(Msg)]
pub enum Msg {
    Cancel,
    EditKeyword(::std::collections::BTreeMap<String, String>),
    Flag,
    Done(Box<::tasks::Task>),
    Ok,
    Set(Box<::tasks::Task>),
    UpdateDate(DateType, Option<::chrono::NaiveDate>),
    UpdateRepeat(Option<::todo_txt::task::Recurrence>),
    UpdatePriority(u8),
}

pub struct Model {
    task: ::tasks::Task,
    relm: ::relm::Relm<Widget>,
}

#[derive(Clone, Copy)]
pub enum DateType {
    Due,
    Threshold,
    Finish,
}

impl Widget {
    fn set_task(&mut self, task: &::tasks::Task) {
        self.model.task = task.clone();

        self.subject.set_text(task.subject.as_str());
        self.priority
            .emit(::widgets::priority::Msg::Set(task.priority));
        self.flag.set_active(task.flagged);
        self.due.emit(::widgets::calendar::Msg::Set(task.due_date));
        self.threshold
            .emit(::widgets::calendar::Msg::Set(task.threshold_date));
        if task.create_date.is_some() {
            self.created
                .emit(::widgets::calendar::Msg::Set(task.create_date));
            self.created.widget().show();
        } else {
            self.created.widget().hide();
        }
        self.repeat
            .emit(::widgets::repeat::Msg::Set(task.recurrence.clone()));
        self.finish
            .emit(::widgets::calendar::Msg::Set(task.finish_date));
        self.keywords
            .emit(::widgets::keywords::Msg::Set(task.tags.clone()));

        let note = task.note.content().unwrap_or_default();
        let buffer = self.note.get_buffer().unwrap();
        buffer.set_text(note.as_str());
    }

    fn get_task(&self) -> ::tasks::Task {
        let mut task = self.model.task.clone();

        task.subject = self.subject.get_text().unwrap();

        let new_note = self.get_note();
        task.note = match task.note {
            ::todo_txt::task::Note::Long { ref filename, .. } => ::todo_txt::task::Note::Long {
                filename: filename.to_string(),
                content: new_note.clone(),
            },
            _ => if new_note.is_empty() {
                ::todo_txt::task::Note::None
            } else {
                ::todo_txt::task::Note::Short(new_note.clone())
            },
        };

        task
    }

    fn get_note(&self) -> String {
        let buffer = match self.note.get_buffer() {
            Some(buffer) => buffer,
            None => return String::new(),
        };
        let start = buffer.get_start_iter();
        let end = buffer.get_end_iter();

        buffer.get_text(&start, &end, false).unwrap_or_default()
    }

    fn edit_keywords(&mut self, keywords: &::std::collections::BTreeMap<String, String>) {
        self.model.task.tags = keywords.clone();
    }

    fn flag(&mut self) {
        self.model.task.flagged = self.flag.get_active();
    }

    fn update_date(&mut self, date_type: DateType, date: Option<::chrono::NaiveDate>) {
        use self::DateType::*;

        match date_type {
            Due => self.model.task.due_date = date,
            Threshold => self.model.task.threshold_date = date,
            Finish => {
                self.model.task.finish_date = date;
                self.model.task.finished = date.is_some();
            }
        }
    }

    fn update_repeat(&mut self, recurrence: &Option<::todo_txt::task::Recurrence>) {
        self.model.task.recurrence = recurrence.clone();
    }

    fn update_priority(&mut self, priority: u8) {
        self.model.task.priority = priority;
    }
}

#[widget]
impl ::relm::Widget for Widget {
    fn init_view(&mut self) {
        self.note.set_property_height_request(150);
        self.created.widget().set_sensitive(false);
    }

    fn model(relm: &::relm::Relm<Self>, _: ()) -> Model {
        Model {
            task: ::tasks::Task::new(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        use self::Msg::*;

        match event {
            Cancel => (),
            Done(_) => (),
            EditKeyword(ref keywords) => self.edit_keywords(keywords),
            Flag => self.flag(),
            Ok => self.model
                .relm
                .stream()
                .emit(Msg::Done(Box::new(self.get_task()))),
            Set(task) => self.set_task(&task),
            UpdateDate(ref date_type, ref date) => self.update_date(*date_type, *date),
            UpdateRepeat(ref recurrence) => self.update_repeat(&recurrence),
            UpdatePriority(priority) => self.update_priority(priority),
        }
    }

    view!
    {
        gtk::ScrolledWindow {
            gtk::Box {
                orientation: ::gtk::Orientation::Vertical,
                spacing: 10,
                gtk::Frame {
                    label: "Subject",
                    #[name="subject"]
                    gtk::Entry {
                        activate => Msg::Ok,
                    },
                },
                gtk::Frame {
                    label: "Priority",
                    gtk::Box {
                        orientation: ::gtk::Orientation::Horizontal,
                        #[name="priority"]
                        Priority {
                            PriorityUpdated(priority) => Msg::UpdatePriority(priority),
                        },
                        #[name="flag"]
                        gtk::ToggleButton {
                            child: {
                                expand: true,
                            },
                            halign: ::gtk::Align::Center,
                            image: &::gtk::Image::new_from_icon_name("emblem-favorite", ::gtk::IconSize::SmallToolbar.into()),
                            tooltip_text: "Flag",
                            toggled => Msg::Flag,
                        },
                    },
                },
                gtk::Frame {
                    label: "Date",
                    gtk::Box {
                        spacing: 10,
                        orientation: ::gtk::Orientation::Vertical,
                        #[name="threshold"]
                        Calendar("Defer until".to_owned()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Threshold, date),
                        },
                        #[name="due"]
                        Calendar("Due".to_owned()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Due, date),
                        },
                        #[name="finish"]
                        Calendar("Completed".to_owned()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Finish, date),
                        },
                        #[name="created"]
                        Calendar("Created".to_owned()),
                    },
                },
                gtk::Frame {
                    label: "Repeat",
                    #[name="repeat"]
                    Repeat {
                        RepeatUpdated(ref recurrence) => Msg::UpdateRepeat(recurrence.clone()),
                    },
                },
                gtk::Frame {
                    label: "Keywords",
                    #[name="keywords"]
                    Keywords {
                        KeywordsUpdated(ref keywords) => Msg::EditKeyword(keywords.clone()),
                    },
                },
                gtk::Frame {
                    label: "Note",
                    #[name="note"]
                    gtk::TextView {
                    },
                },
                gtk::ActionBar {
                    child: {
                        pack_type: ::gtk::PackType::End,
                    },
                    gtk::ButtonBox {
                        orientation: ::gtk::Orientation::Horizontal,
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
