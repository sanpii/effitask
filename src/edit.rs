use gtk;
use gtk::prelude::*;
use relm_attributes::widget;
use widgets::keywords::Msg::Updated as KeywordsUpdated;
use widgets::calendar::Msg::Updated as CalendarUpdated;
use widgets::repeat::Msg::Updated as RepeatUpdated;

#[derive(Msg)]
pub enum Msg {
    Cancel,
    EditKeyword(::std::collections::BTreeMap<String, String>),
    Done(::tasks::Task),
    Ok,
    Set(::tasks::Task),
    UpdateDate(DateType, Option<::chrono::NaiveDate>),
    UpdateRepeat(Option<::tasks::Recurrence>),
    UpdatePriority,
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

impl Widget
{
    fn set_task(&mut self, task: &::tasks::Task)
    {
        self.model.task = task.clone();

        self.subject.set_text(task.subject.as_str());
        self.priority.set_value(f64::from(task.priority));
        self.due.emit(::widgets::calendar::Msg::Set(task.due_date));
        self.threshold.emit(::widgets::calendar::Msg::Set(task.threshold_date));
        if task.create_date.is_some() {
            self.created.emit(::widgets::calendar::Msg::Set(task.create_date));
            self.created.widget()
                .show();
        }
        else {
            self.created.widget()
                .hide();
        }
        self.repeat.emit(::widgets::repeat::Msg::Set(task.recurrence.clone()));
        self.finish.emit(::widgets::calendar::Msg::Set(task.finish_date));
        self.keywords.emit(::widgets::keywords::Msg::Set(task.tags.clone()));

        let note = task.note.content()
            .unwrap_or_default();
        let buffer = self.note.get_buffer()
            .unwrap();
        buffer.set_text(note.as_str());
    }

    fn get_task(&self) -> ::tasks::Task
    {
        let mut task = self.model.task.clone();

        task.subject = self.subject.get_text()
            .unwrap();

        let new_note = self.get_note();
        task.note = match task.note {
            ::tasks::Note::Long { filename, .. } => ::tasks::Note::Long {
                filename,
                content: new_note.clone()
            },
            _ => if new_note.is_empty() {
                ::tasks::Note::None
            }
            else {
                ::tasks::Note::Short(new_note.clone())
            },
        };

        task
    }

    fn get_note(&self) -> String
    {
        let buffer = match self.note.get_buffer() {
            Some(buffer) => buffer,
            None => return String::new(),
        };
        let start = buffer.get_start_iter();
        let end = buffer.get_end_iter();

        buffer.get_text(&start, &end, false)
            .unwrap_or_default()
    }

    fn edit_keywords(&mut self, keywords: &::std::collections::BTreeMap<String, String>)
    {
        self.model.task.tags = keywords.clone();
    }

    fn update_date(&mut self, date_type: &DateType, date: &Option<::chrono::NaiveDate>)
    {
        use self::DateType::*;

        match *date_type {
            Due => self.model.task.due_date = *date,
            Threshold => self.model.task.threshold_date = *date,
            Finish => {
                self.model.task.finish_date = *date;
                self.model.task.finished = date.is_some();
            },
        }
    }

    fn update_repeat(&mut self, recurrence: &Option<::tasks::Recurrence>)
    {
        self.model.task.recurrence = recurrence.clone();
    }

    fn update_priority(&mut self)
    {
        self.model.task.priority = self.priority.get_value() as u8;
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.note.set_property_height_request(150);
        self.priority.set_adjustment(&::gtk::Adjustment::new(0., 0., 27., 1., 5., 1.));
        self.created.widget().set_sensitive(false);
    }

    fn model(relm: &::relm::Relm<Self>,_: ()) -> Model
    {
        Model {
            task: ::tasks::Task::new(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Cancel => (),
            Done(_) => (),
            EditKeyword(ref keywords) => self.edit_keywords(keywords),
            Ok => self.model.relm.stream().emit(Msg::Done(self.get_task())),
            Set(task) => self.set_task(&task),
            UpdateDate(ref date_type, ref date) => self.update_date(date_type, &date),
            UpdateRepeat(ref recurrence) => self.update_repeat(&recurrence),
            UpdatePriority => self.update_priority(),
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
                    #[name="priority"]
                    gtk::SpinButton {
                        focus_out_event(_, _) => (Msg::UpdatePriority, ::gtk::Inhibit(false)),
                    },
                },
                gtk::Frame {
                    label: "Date",
                    gtk::Box {
                        spacing: 10,
                        orientation: ::gtk::Orientation::Vertical,
                        #[name="threshold"]
                        ::widgets::Calendar("Defer until".to_owned()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Threshold, date),
                        },
                        #[name="due"]
                        ::widgets::Calendar("Due".to_owned()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Due, date),
                        },
                        #[name="finish"]
                        ::widgets::Calendar("Completed".to_owned()) {
                            CalendarUpdated(date) => Msg::UpdateDate(DateType::Finish, date),
                        },
                        #[name="created"]
                        ::widgets::Calendar("Created".to_owned()),
                    },
                },
                gtk::Frame {
                    label: "Repeat",
                    #[name="repeat"]
                    ::widgets::Repeat {
                        RepeatUpdated(ref recurrence) => Msg::UpdateRepeat(recurrence.clone()),
                    },
                },
                gtk::Frame {
                    label: "Keywords",
                    #[name="keywords"]
                    ::widgets::Keywords {
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
                    packing: {
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
