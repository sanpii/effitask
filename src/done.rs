use crate::widgets::tasks::Msg::{Complete, Edit};
use crate::widgets::Tasks;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    Update,
}

impl Widget {
    fn update_tasks(&mut self) {
        let list = crate::application::tasks();
        let tasks = list.tasks.iter().filter(|x| x.finished).cloned().collect();

        self.tasks.emit(crate::widgets::tasks::Msg::Update(tasks));
    }
}

#[relm_attributes::widget]
impl relm::Widget for Widget {
    fn model(_: ()) {}

    fn update(&mut self, event: Msg) {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update => self.update_tasks(),
        }
    }

    view! {
        #[name="tasks"]
        Tasks {
            Complete(ref task) => Msg::Complete(task.clone()),
            Edit(ref task) => Msg::Edit(task.clone()),
        }
    }
}
