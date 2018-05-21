use relm_attributes::widget;
use widgets::tasks::Msg::{Complete, Edit};
use widgets::Tasks;

#[derive(Msg)]
pub enum Msg {
    Complete(Box<::tasks::Task>),
    Edit(Box<::tasks::Task>),
    Update,
}

impl Widget {
    fn update_tasks(&self) {
        let today = ::date::today();

        let list = ::application::tasks();
        let preferences = ::application::preferences();
        let tasks = list.tasks
            .iter()
            .filter(|x| {
                !x.finished && x.projects.is_empty()
                    && (preferences.defered || x.threshold_date.is_none()
                        || x.threshold_date.unwrap() <= today)
            })
            .cloned()
            .collect();

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget {
    fn model() -> () {}

    fn update(&mut self, event: Msg) {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update => self.update_tasks(),
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
