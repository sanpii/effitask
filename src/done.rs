use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Update(::tasks::List),
}

impl Widget
{
    fn update_tasks(&mut self, list: ::tasks::List)
    {
        let tasks = list.tasks.iter()
            .filter(|x| x.finished)
            .map(|x| x.clone())
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
            Update(list) => self.update_tasks(list),
        }
    }

    view!
    {
        #[name="tasks"]
        ::widgets::Tasks {
        }
    }
}
