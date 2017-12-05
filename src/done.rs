use relm_attributes::widget;

impl Widget
{
    fn populate(&mut self)
    {
        self.tasks.emit(::widgets::tasks::Msg::Update(self.model.done.clone()));
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
