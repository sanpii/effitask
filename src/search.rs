use crate::widgets::tasks::Msg::{Complete, Edit};
use crate::widgets::Tasks;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    Update,
    UpdateFilter(String),
}

impl Widget {
    fn update_tasks(&mut self) {
        self.update();
    }

    fn update_filter(&mut self, filter: &str) {
        self.model = filter.to_string();
        self.update();
    }

    fn update(&self) {
        let filter = self.model.to_lowercase();
        let list = crate::application::tasks();

        let tasks = list
            .tasks
            .iter()
            .filter(|x| x.subject.to_lowercase().contains(filter.as_str()))
            .cloned()
            .collect();

        self.components
            .tasks
            .emit(crate::widgets::tasks::Msg::Update(tasks));
    }
}

#[relm_derive::widget]
impl relm::Widget for Widget {
    fn model(_: ()) -> String {
        String::new()
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Complete(_) | Edit(_) => (),
            Update => self.update_tasks(),
            UpdateFilter(filter) => self.update_filter(&filter),
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
