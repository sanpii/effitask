use gtk;
use gtk::prelude::*;
use relm::ContainerWidget;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Edit(::tasks::Task),
    Complete(::tasks::Task),
    Update(Vec<::tasks::Task>),
}

pub struct Model {
    children: Vec<::relm::Component<super::Task>>,
    relm: ::relm::Relm<Tasks>,
}

impl Tasks
{
    fn update_tasks(&mut self, tasks: Vec<::tasks::Task>)
    {
        self.clear();

        if tasks.is_empty() {
            self.list_box.hide();
            self.label.show();
        }
        else {
            self.list_box.show();
            self.label.hide();

            for task in tasks.iter() {
                let child = self.list_box.add_widget::<super::Task, _>(&self.model.relm, task.clone());
                connect!(child@::widgets::task::Msg::Complete(ref task), self.model.relm, Msg::Complete(task.clone()));
                connect!(child@::widgets::task::Msg::Edit(ref task), self.model.relm, Msg::Edit(task.clone()));

                self.model.children.push(child);
            }
        }
    }

    fn clear(&mut self)
    {
        for child in self.list_box.get_children() {
            self.list_box.remove(&child);
        }
        self.model.children = Vec::new();
    }
}

#[widget]
impl ::relm::Widget for Tasks
{
    fn model(relm: &::relm::Relm<Self>, _: ()) -> Model
    {
        Model {
            children: Vec::new(),
            relm: relm.clone()
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update(tasks) => self.update_tasks(tasks),
        }
    }

    view!
    {
        gtk::Box {
            #[name="list_box"]
            gtk::ListBox {
                padding: {
                    fill: true,
                    expand: true,
                },
            },
            #[name="label"]
            gtk::Label {
                padding: {
                    fill: true,
                    expand: true,
                },
                text: "Nothing to do :)",
            },
        }
    }
}
