use relm_attributes::widget;
use widgets::tasks::Msg::{Complete, Edit};

#[derive(Msg)]
pub enum Msg {
    Complete(Box<::tasks::Task>),
    Edit(Box<::tasks::Task>),
    Update(bool, bool),
    UpdateFilter(String),
}

pub struct Model {
    list: ::tasks::List,
    defered: bool,
    done: bool,
    filter: String,
}

impl Widget
{
    fn update_tasks(&mut self, defered: bool, done: bool)
    {
        self.model.list = ::application::tasks();
        self.model.defered = defered;
        self.model.done = done;
        self.update();
    }

    fn update_filter(&mut self, filter: &str)
    {
        self.model.filter = filter.to_string();
        self.update();
    }

    fn update(&self)
    {
        let filter = self.model.filter.to_lowercase();

        let tasks = self.model.list.tasks.iter()
            .filter(|x| x.subject.to_lowercase().contains(filter.as_str()))
            .cloned()
            .collect();

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn model(_: ()) -> Model
    {
        Model {
            list: ::tasks::List::new(),
            filter: String::new(),
            defered: false,
            done: false,
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update(defered, done) => self.update_tasks(defered, done),
            UpdateFilter(filter) => self.update_filter(&filter),
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
