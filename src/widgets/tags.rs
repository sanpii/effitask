use relm_attributes::widget;
use widgets::filter::Msg::Filter;

pub enum Type {
    Projects,
    Contexts,
}

#[derive(Msg)]
pub enum Msg {
    UpdateFilter(Option<String>),
}

pub struct Model {
    list: ::tasks::List,
    tags: Type,
}

impl Tags
{
    fn populate_tags(&mut self)
    {
        let tags = match self.model.tags {
            Type::Projects => self.model.list.projects(),
            Type::Contexts => self.model.list.contexts(),
        };

        let tags = tags.iter()
            .map(|x| (x.clone(), self.get_progress(x)))
            .filter(|&(_, progress)| progress < 100)
            .collect();

        self.filter.emit(::widgets::filter::Msg::UpdateFilters(tags));
    }

    fn populate_tasks(&mut self, filter: Option<String>)
    {
        let today = ::chrono::Local::now()
            .date()
            .naive_local();

        let tasks = self.model.list.tasks.iter()
            .filter(|x| {
                let tags = self.get_tags(x);

                !x.finished
                    && !tags.is_empty()
                    && (filter.is_none() || tags.contains(&filter.clone().unwrap()))
                    && (x.threshold_date.is_none() || x.threshold_date.unwrap() <= today)
            })
            .map(|x| x.clone())
            .collect();

        self.filter.emit(::widgets::filter::Msg::UpdateTasks(tasks));
    }

    fn get_progress(&self, tag: &String) -> u32
    {
        let (done, total) = self.model.list.tasks.iter()
            .filter(|x| self.get_tags(x).contains(tag))
            .fold((0., 0.), |(mut done, total), x| {
                if x.finished {
                    done += 1.;
                }

                (done, total + 1.)
            });

        (done / total * 100.) as u32
    }

    fn get_tags<'a>(&self, task: &'a ::tasks::Task) -> &'a Vec<String>
    {
        match self.model.tags {
            Type::Projects => &task.projects,
            Type::Contexts => &task.contexts,
        }
    }
}

#[widget]
impl ::relm::Widget for Tags
{
    fn init_view(&mut self)
    {
        self.populate_tags();
        self.populate_tasks(None);
    }

    fn model((list, tags): (::tasks::List, Type)) -> Model
    {
        Model {
            list,
            tags,
        }
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
