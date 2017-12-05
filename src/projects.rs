use relm_attributes::widget;
use widgets::filter_panel::Msg::Filter;

#[derive(Msg)]
pub enum Msg {
    UpdateFilter(Option<String>),
}

impl Widget
{
    fn populate_projects(&mut self)
    {
        let projects = self.model.projects();

        self.filter_panel.emit(::widgets::filter_panel::Msg::UpdateFilters(projects));
    }

    fn populate_tasks(&mut self, filter: Option<String>)
    {
        let mut tasks = Vec::new();

        for task in self.model.todo.iter() {
            if !task.projects.is_empty() && (filter.is_none() || task.projects.contains(&filter.clone().unwrap())) {
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
        self.populate_projects();
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
