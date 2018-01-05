use relm_attributes::widget;
use widgets::tasks::Msg::{Complete, Edit};

#[derive(Msg)]
pub enum Msg {
    Complete(::tasks::Task),
    Edit(::tasks::Task),
    Update(::tasks::List, bool, bool),
}

impl Widget
{
    fn update_tasks(&mut self, list: &::tasks::List, defered: bool, done: bool)
    {
        let today = ::chrono::Local::now()
            .date()
            .naive_local();

        let tasks = list.tasks.iter()
            .filter(|x| {
                x.flagged
                    && (done || !x.finished)
                    && (defered || x.threshold_date.is_none() || x.threshold_date.unwrap() <= today)
            })
            .cloned()
            .collect();

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn model(_: ()) -> ()
    {
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update(list, defered, done) => self.update_tasks(&list, defered, done),
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
