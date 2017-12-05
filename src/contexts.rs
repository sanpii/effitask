use relm_attributes::widget;
use widgets::filter_panel::Msg::Filter;

#[derive(Msg)]
pub enum Msg {
    UpdateFilter(Option<String>),
}

impl Widget
{
    fn populate_contexts(&mut self)
    {
        let contexts = self.model.contexts();

        self.filter_panel.emit(::widgets::filter_panel::Msg::UpdateFilters(contexts));
    }

    fn populate_tasks(&mut self, filter: Option<String>)
    {
        let mut tasks = Vec::new();

        for task in self.model.todo.iter() {
            if !task.contexts.is_empty() && (filter.is_none() || task.contexts.contains(&filter.clone().unwrap())) {
                tasks.push(task.clone());
            }
        }

        self.filter_panel.emit(::widgets::filter_panel::Msg::UpdateTasks(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.populate_contexts();
        self.populate_tasks(None);
    }

    fn model(tasks: ::tasks::List) -> ::tasks::List
    {
        tasks
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            UpdateFilter(filter) =>  self.populate_tasks(filter),
        }
    }

    view!
    {
        #[name="filter_panel"]
        ::widgets::FilterPanel {
            Filter(ref filter) => Msg::UpdateFilter(filter.clone()),
        }
    }
}
