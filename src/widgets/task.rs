use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Click(::gdk::EventButton),
    Complete(::tasks::Task),
    Edit(::tasks::Task),
    ShowNote,
    Toggle,
}

pub struct Model {
    note_label: ::gtk::Label,
    note: ::gtk::Popover,
    task: ::tasks::Task,
    relm: ::relm::Relm<Task>,
}

#[widget]
impl ::relm::Widget for Task
{
    fn init_view(&mut self)
    {
        let task = &self.model.task;

        let context = self.root()
            .get_style_context()
            .unwrap();

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
        }
        else {
            self.note_button.hide();
        }

        if task.tags.len() > 0 {
            let text = task.tags.iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join(" · ");

            self.keywords_label.set_text(&text);
        }
        else {
            self.keywords.hide();
        }

        if let Some(due) = task.due_date {
            let today = ::chrono::Local::now()
                .date()
                .naive_local();

            let date = if due == today {
                String::from("today")
            }
            else if due == today.pred() {
                String::from("yesterday")
            }
            else if due == today.succ() {
                String::from("tomorrow")
            }
            else {
                due.format("%Y-%m-%d")
                    .to_string()
            };

            self.date_label.set_text(format!("due: {}", date).as_str());
        }
        else {
            self.date.hide();
        }
    }

    fn model(relm: &::relm::Relm<Self>, task: ::tasks::Task) -> Model
    {
        let note_label = ::gtk::Label::new(None);
        note_label.show();

        if let Some(ref note) = task.note.content() {
            note_label.set_text(note);
        }

        let note = ::gtk::Popover::new(None::<&::gtk::Button>);
        note.set_position(::gtk::PositionType::Right);

        Model {
            note_label,
            note,
            task,
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Click(event) => if event.get_event_type() == ::gdk::EventType::DoubleButtonPress {
                self.model.relm.stream().emit(Edit(self.model.task.clone()))
            },
            Complete(_) => (),
            Edit(_) => (),
            ShowNote => self.model.note.popup(),
            Toggle => self.model.relm.stream().emit(Complete(self.model.task.clone())),
        }
    }

    view!
    {
        gtk::EventBox {
            button_press_event(_, event) => (Msg::Click(event.clone()), ::gtk::Inhibit(false)),
            gtk::Box {
                orientation: ::gtk::Orientation::Vertical,
                gtk::Box {
                    spacing: 5,
                    orientation: ::gtk::Orientation::Horizontal,
                    gtk::CheckButton {
                        active: self.model.task.finished,
                        toggled => Msg::Toggle,
                    },
                    gtk::Label {
                        packing: {
                            expand: true,
                            fill: true,
                        },
                        label: self.model.task.subject.as_str(),
                        xalign: 0.,
                    },
                    #[name="note_button"]
                    gtk::Button {
                        image: &::gtk::Image::new_from_icon_name("text-x-generic", ::gtk::IconSize::LargeToolbar.into()),
                        clicked => Msg::ShowNote,
                    },
                },
                gtk::Box {
                    spacing: 5,
                    orientation: ::gtk::Orientation::Horizontal,
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
                        packing: {
                            pack_type: ::gtk::PackType::End,
                        },
                        #[name="date_label"]
                        gtk::Label {
                        },
                    },
                },
            },
        }
    }
}
