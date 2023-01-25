use gtk::prelude::*;
use relm::ContainerWidget;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Edit(Box<crate::tasks::Task>),
    Complete(Box<crate::tasks::Task>),
    Update(Vec<crate::tasks::Task>),
}

pub struct Model {
    children: Vec<relm::Component<super::Task>>,
    relm: relm::Relm<Tasks>,
}

impl Tasks {
    fn update_tasks(&mut self, tasks: &[crate::tasks::Task]) {
        self.clear();

        if tasks.is_empty() {
            self.widgets.list_box.hide();
            self.widgets.label.show();
        } else {
            self.widgets.list_box.show();
            self.widgets.label.hide();

            let mut sorted_tasks = tasks.to_owned();
            sorted_tasks.sort();
            sorted_tasks.reverse();

            for task in &mut sorted_tasks {
                task.subject = format!("{} - Test", task.subject);
                let child = self
                    .widgets
                    .list_box
                    .add_widget::<super::Task>(task.clone());

                relm::connect!(
                    child@crate::widgets::task::Msg::Complete(ref task),
                    self.model.relm,
                    Msg::Complete(task.clone())
                );
                relm::connect!(
                    child@crate::widgets::task::Msg::Edit(ref task),
                    self.model.relm,
                    Msg::Edit(task.clone())
                );

                self.model.children.push(child);
            }
        }
    }

    fn clear(&mut self) {
        for child in self.widgets.list_box.children() {
            self.widgets.list_box.remove(&child);
        }
        self.model.children = Vec::new();
    }
}

#[relm_derive::widget]
impl relm::Widget for Tasks {
    fn model(relm: &relm::Relm<Self>, _: ()) -> Model {
        Model {
            children: Vec::new(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update(tasks) => self.update_tasks(&tasks),
        }
    }

    view! {
        gtk::ScrolledWindow {
            gtk::Box {
                #[name="list_box"]
                gtk::ListBox {
                    child: {
                        fill: true,
                        expand: true,
                    },
                },
                #[name="label"]
                gtk::Label {
                    child: {
                        fill: true,
                        expand: true,
                    },
                    text: "Nothing to do :)",
                },
            }
        }
    }
}
