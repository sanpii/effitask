use relm_attributes::widget;
use widgets::tasks::Msg::{Complete, Edit};

#[derive(Msg)]
pub enum Msg {
    Complete(::tasks::Task),
    Edit(::tasks::Task),
    Update(::tasks::List),
}

impl Widget
{
    fn update_tasks(&self, list: &::tasks::List)
    {
        let today = ::chrono::Local::now()
            .date()
            .naive_local();

        let tasks = list.tasks.iter()
            .filter(|x| {
                !x.finished
                    && x.projects.is_empty()
                    && (x.threshold_date.is_none() || x.threshold_date.unwrap() <= today)
            })
            .cloned()
            .collect();

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn model() -> ()
    {
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update(list) => self.update_tasks(&list),
        }
    }

    view!
    {
        #[name="tasks"]
        ::widgets::Tasks {
            Complete(ref task) => Msg::Complete(task.clone()),
            Edit(ref task) => Msg::Edit(task.clone()),
        }
    }
}
