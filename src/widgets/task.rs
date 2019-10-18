use crate::widgets::Circle;
use gtk::prelude::*;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Click(gdk::EventButton),
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    ShowNote,
    Toggle,
}

pub struct Model {
    note_label: gtk::Label,
    note: gtk::Popover,
    task: crate::tasks::Task,
    relm: relm::Relm<Task>,
}

#[relm_attributes::widget]
impl relm::Widget for Task {
    fn init_view(&mut self) {
        use gtk::StyleContextExt;

        let task = &self.model.task;

        let context = self.root().get_style_context();

        context.add_class("task");

        if task.finished {
            context.add_class("finished");
        }

        if task.priority < 26 {
            let priority = (b'a' + task.priority) as char;
            context.add_class(format!("pri_{}", priority).as_str());
        }

        let note = task.note.content();
        if note.is_some() {
            self.model.note.set_relative_to(Some(&self.note_button));
            self.model.note.add(&self.model.note_label);
        } else {
            self.note_button.hide();
        }

        if !task.tags.is_empty() {
            let text = task
                .tags
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join(" · ");

            self.keywords_label.set_text(&text);
        } else {
            self.keywords.hide();
        }

        let context = self.date.get_style_context();
        context.add_class("date");

        if let Some(threshold) = task.threshold_date {
            let context = self.threshold_label.get_style_context();
            context.add_class("threshold");

            let date = self.date_alias(threshold);
            self.threshold_label
                .set_text(format!("Deferred until {}", date).as_str());
        } else {
            self.threshold_label.hide();
        }

        if task.threshold_date.is_some() && task.due_date.is_some() {
            self.arrow_label.show();
        } else {
            self.arrow_label.hide();
        }

        if let Some(due) = task.due_date {
            let context = self.due_label.get_style_context();
            context.add_class("due");

            let date = self.date_alias(due);

            let today = crate::date::today();

            if due < today {
                context.add_class("past");
            }

            self.due_label.set_text(format!("due: {}", date).as_str());
        } else {
            self.due_label.hide();
        }
    }

    fn date_alias(&self, date: chrono::NaiveDate) -> String {
        let today = crate::date::today();

        if date == today {
            String::from("today")
        } else if date == today.pred() {
            String::from("yesterday")
        } else if date == today.succ() {
            String::from("tomorrow")
        } else {
            date.format("%Y-%m-%d").to_string()
        }
    }

    fn model(relm: &relm::Relm<Self>, task: crate::tasks::Task) -> Model {
        use crate::tasks::Markup;

        let note_label = gtk::Label::new(None);
        note_label.show();

        if let Some(ref note) = task.note.markup() {
            note_label.set_markup(note);
        }

        let note = gtk::Popover::new(None::<&gtk::Button>);
        note.set_position(gtk::PositionType::Right);

        Model {
            note_label,
            note,
            task,
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        use self::Msg::*;

        match event {
            Click(event) => {
                if event.get_event_type() == gdk::EventType::DoubleButtonPress {
                    self.model
                        .relm
                        .stream()
                        .emit(Edit(Box::new(self.model.task.clone())))
                }
            }
            Complete(_) => (),
            Edit(_) => (),
            ShowNote => self.model.note.popup(),
            Toggle => self
                .model
                .relm
                .stream()
                .emit(Complete(Box::new(self.model.task.clone()))),
        }
    }

    view! {
        gtk::EventBox {
            button_press_event(_, event) => (Msg::Click(event.clone()), gtk::Inhibit(false)),
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,
                spacing: 5,
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    child: {
                        expand: true,
                        fill: true,
                    },
                    gtk::Box {
                        spacing: 5,
                        orientation: gtk::Orientation::Horizontal,
                        gtk::CheckButton {
                            active: self.model.task.finished,
                            toggled => Msg::Toggle,
                        },
                        gtk::Label {
                            child: {
                                expand: true,
                                fill: true,
                            },
                            markup: self.model.task.markup_subject().as_str(),
                            xalign: 0.,
                        },
                    },
                    gtk::Box {
                        spacing: 5,
                        orientation: gtk::Orientation::Horizontal,
                        #[name="note_button"]
                        gtk::Button {
                            image: Some(&gtk::Image::new_from_icon_name(Some("text-x-generic"), gtk::IconSize::LargeToolbar)),
                            clicked => Msg::ShowNote,
                        },
                        #[name="keywords"]
                        gtk::Box {
                            gtk::Image {
                                property_icon_name: Some("mail-attachment"),
                            },
                            #[name="keywords_label"]
                            gtk::Label {
                            },
                        },
                        #[name="date"]
                        gtk::Box {
                            spacing: 5,
                            child: {
                                pack_type: gtk::PackType::End,
                            },
                            #[name="threshold_label"]
                            gtk::Label {
                            },
                            #[name="arrow_label"]
                            gtk::Label {
                                text: " ➡ ",
                            },
                            #[name="due_label"]
                            gtk::Label {
                            },
                        },
                    },
                },
                #[name="circle"]
                Circle(self.model.task.clone()) {
                    child: {
                        expand: false,
                        fill: true,
                    },
                },
            },
        }
    }
}
