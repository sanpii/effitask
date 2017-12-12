use gtk;
use gtk::prelude::*;
use relm_attributes::widget;
use widgets::keywords::Msg::Updated as KeywordsUpdated;
use widgets::calendar::Msg::Updated as CalendarUpdated;

#[derive(Msg)]
pub enum Msg {
    Cancel,
    EditKeyword(::std::collections::BTreeMap<String, String>),
    UpdateDate(DateType, Option<::chrono::NaiveDate>),
    Done(::tasks::Task),
    Ok,
    Set(::tasks::Task),
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
    fn set_task(&mut self, task: ::tasks::Task)
    {
        self.model.task = task.clone();

        self.subject.set_text(task.subject.as_str());
        self.due.emit(::widgets::calendar::Msg::Set(task.due_date));
        self.threshold.emit(::widgets::calendar::Msg::Set(task.threshold_date));
        self.finish.emit(::widgets::calendar::Msg::Set(task.finish_date));
        self.keywords.emit(::widgets::keywords::Msg::Set(task.tags.clone()));

        let note = task.note.content()
            .unwrap_or(String::new());
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
            ::tasks::Note::Long { filename, content: _ } => ::tasks::Note::Long {
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
            .unwrap_or(String::new())
    }

    fn edit_keywords(&mut self, keywords: &::std::collections::BTreeMap<String, String>)
    {
        self.model.task.tags = keywords.clone();
    }

    fn update_date(&mut self, date_type: &DateType, date: &Option<::chrono::NaiveDate>)
    {
        use self::DateType::*;

        match *date_type {
            Due => self.model.task.due_date = date.clone(),
            Threshold => self.model.task.threshold_date = date.clone(),
            Finish => {
                self.model.task.finish_date = date.clone();
                self.model.task.finished = date.is_some();
            },
        }
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.note.set_property_height_request(250);
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
            Set(task) => self.set_task(task),
            UpdateDate(ref date_type, ref date) => self.update_date(date_type, &date),
        }
    }

    view!
    {
        gtk::Box {
            orientation: ::gtk::Orientation::Vertical,
            spacing: 10,
            gtk::Frame {
                label: "Subject",
                #[name="subject"]
                gtk::Entry {
                },
            },
            gtk::Frame {
                label: "Date",
                gtk::Box {
                    orientation: ::gtk::Orientation::Horizontal,
                    gtk::Box {
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
                    },
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
