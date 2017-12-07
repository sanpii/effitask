use relm_attributes::widget;
use widgets::filter::Msg::Filter;

#[derive(Msg)]
pub enum Msg {
    UpdateFilter(Option<String>),
}

impl Widget
{
    fn populate_projects(&mut self)
    {
        let projects = self.model.projects()
            .iter()
            .map(|x| (x.clone(), self.get_progress(x)))
            .filter(|&(_, progress)| progress < 100)
            .collect();

        self.filter.emit(::widgets::filter::Msg::UpdateFilters(projects));
    }

    fn populate_tasks(&mut self, filter: Option<String>)
    {
        let tasks = self.model.tasks.iter()
            .filter(|x| {
                !x.finished
                    && !x.projects.is_empty()
                    && (filter.is_none() || x.projects.contains(&filter.clone().unwrap()))
            })
            .map(|x| x.clone())
            .collect();

        self.filter.emit(::widgets::filter::Msg::UpdateTasks(tasks));
    }

    fn get_progress(&self, project: &String) -> u32
    {
        let (done, total) = self.model.tasks.iter()
            .filter(|x| x.projects.contains(project))
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
        #[name="filter"]
        ::widgets::Filter {
            Filter(ref filter) => Msg::UpdateFilter(filter.clone()),
        }
    }
}
