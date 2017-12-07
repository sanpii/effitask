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
        let contexts = self.model.contexts()
            .iter()
            .map(|x| (x.clone(), self.get_progress(x)))
            .filter(|&(_, progress)| progress < 100)
            .collect();

        self.filter_panel.emit(::widgets::filter_panel::Msg::UpdateFilters(contexts));
    }

    fn populate_tasks(&mut self, filter: Option<String>)
    {
        let tasks = self.model.tasks.iter()
            .filter(|x| {
                !x.finished
                    && !x.contexts.is_empty()
                    && (filter.is_none() || x.contexts.contains(&filter.clone().unwrap()))
            })
            .map(|x| x.clone())
            .collect();

        self.filter_panel.emit(::widgets::filter_panel::Msg::UpdateTasks(tasks));
    }

    fn get_progress(&self, context: &String) -> u32
    {
        let (done, total) = self.model.tasks.iter()
            .filter(|x| x.projects.contains(context))
            .fold((0., 0.), |(mut done, total), x| {
                if x.finished {
                    done += 1.;
                }

                (done, total + 1.)
            });

        (done / total * 100.) as u32
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
