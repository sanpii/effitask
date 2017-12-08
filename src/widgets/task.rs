use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[widget]
impl ::relm::Widget for Task
{
    fn init_view(&mut self)
    {
        let task = &self.model;

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
    }

    fn model(task: ::tasks::Task) -> ::tasks::Task
    {
        task
    }

    fn update(&mut self, _: ())
    {
    }

    view!
    {
        gtk::Box {
            spacing: 5,
            gtk::CheckButton {
                active: self.model.finished,
                sensitive: false,
            },
            gtk::Label {
                label: self.model.subject.as_str(),
            },
        }
    }
}
