use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    ShowNote,
}

pub struct Model {
    note_label: ::gtk::Label,
    note: ::gtk::Popover,
    task: ::tasks::Task,
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

        if task.note.is_some() {
            self.model.note.set_relative_to(Some(&self.note_button));
            self.model.note.add(&self.model.note_label);
        }
        else {
            self.note_button.hide();
        }
    }

    fn model(task: ::tasks::Task) -> Model
    {
        let note_label = ::gtk::Label::new(None);
        note_label.show();

        if let Some(ref note) = task.note {
            note_label.set_text(note);
        }

        let note = ::gtk::Popover::new(None::<&::gtk::Button>);
        note.set_position(::gtk::PositionType::Right);

        Model {
            note_label,
            note,
            task,
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            ShowNote => self.model.note.popup(),
        }
    }

    view!
    {
        gtk::Box {
            spacing: 5,
            gtk::CheckButton {
                active: self.model.task.finished,
                sensitive: false,
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
        }
    }
}
