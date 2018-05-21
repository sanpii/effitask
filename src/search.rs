use relm_attributes::widget;
use widgets::tasks::Msg::{Complete, Edit};
use widgets::Tasks;

#[derive(Msg)]
pub enum Msg {
    Complete(Box<::tasks::Task>),
    Edit(Box<::tasks::Task>),
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
        let list = ::application::tasks();

        let tasks = list.tasks
            .iter()
            .filter(|x| x.subject.to_lowercase().contains(filter.as_str()))
            .cloned()
            .collect();

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget {
    fn model(_: ()) -> String {
        String::new()
    }

    fn update(&mut self, event: Msg) {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update => self.update_tasks(),
            UpdateFilter(filter) => self.update_filter(&filter),
        }
    }

    view!
    {
        #[name="tasks"]
        Tasks {
            Complete(ref task) => Msg::Complete(task.clone()),
            Edit(ref task) => Msg::Edit(task.clone()),
        }
    }
}
