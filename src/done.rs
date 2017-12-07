use relm_attributes::widget;

impl Widget
{
    fn populate(&mut self)
    {
        let tasks = self.model.tasks.iter()
            .filter(|x| x.finished)
            .map(|x| x.clone())
            .collect();

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.populate();
    }

    fn model(tasks: ::tasks::List) -> ::tasks::List
    {
        tasks
    }

    fn update(&mut self, _: ())
    {
    }

    view!
    {
        #[name="tasks"]
        ::widgets::Tasks {
        }
    }
}
