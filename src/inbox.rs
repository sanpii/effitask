use relm_attributes::widget;

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        let today = ::chrono::Local::now()
            .date()
            .naive_local();

        let tasks = self.model.tasks.iter()
            .filter(|x| {
                !x.finished
                    && x.projects.is_empty()
                    && (x.threshold_date.is_none() || x.threshold_date.unwrap() <= today)
            })
            .map(|x| x.clone())
            .collect();

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
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
