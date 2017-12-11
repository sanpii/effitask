use relm_attributes::widget;
use widgets::filter::Msg::{Complete, Filters};

#[derive(Clone, Copy)]
pub enum Type {
    Projects,
    Contexts,
}

#[derive(Msg)]
pub enum Msg {
    Complete(::tasks::Task),
    UpdateFilters(Vec<String>),
    Update(::tasks::List),
}

pub struct Model {
    list: ::tasks::List,
    tag: Type,
}

impl Tags
{
    fn update_tags(&self, tag: Type, list: &::tasks::List)
    {
        let tags = match tag {
            Type::Projects => list.projects(),
            Type::Contexts => list.contexts(),
        };

        let tags = tags.iter()
            .map(|x| (x.clone(), self.get_progress(tag, list, x)))
            .filter(|&(_, (done, total))| done != total)
            .collect();

        self.filter.emit(::widgets::filter::Msg::UpdateFilters(tags));
    }

    fn get_progress(&self, tag: Type, list: &::tasks::List, current: &String) -> (u32, u32)
    {
        list.tasks.iter()
            .filter(|x| self.get_tags(tag, x).contains(current))
            .fold((0, 0), |(mut done, total), x| {
                if x.finished {
                    done += 1;
                }

                (done, total + 1)
            })
    }

    fn update_tasks(&self, tag: Type, list: &::tasks::List, filters: Vec<String>)
    {
        let today = ::chrono::Local::now()
            .date()
            .naive_local();

        let tasks = list.tasks.iter()
            .filter(|x| {
                let tags = self.get_tags(tag, x);

                !x.finished
                    && !tags.is_empty()
                    && Self::has_filter(tags, &filters)
                    && (x.threshold_date.is_none() || x.threshold_date.unwrap() <= today)
            })
            .map(|x| x.clone())
            .collect();

        self.filter.emit(::widgets::filter::Msg::UpdateTasks(tasks));
    }

    fn get_tags<'a>(&self, tag: Type, task: &'a ::tasks::Task) -> &'a Vec<String>
    {
        match tag {
            Type::Projects => &task.projects,
            Type::Contexts => &task.contexts,
        }
    }

    fn has_filter(tags: &Vec<String>, filters: &Vec<String>) -> bool
    {
        for filter in filters {
            if tags.contains(filter) {
                return true;
            }
        }

        false
    }
}

#[widget]
impl ::relm::Widget for Tags
{
    fn model(tag: Type) -> Model
    {
        Model {
            list: ::tasks::List::new(),
            tag: tag,
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Update(list) =>  {
                self.model.list = list.clone();

                self.update_tags(self.model.tag, &self.model.list);
                self.update_tasks(self.model.tag, &self.model.list, Vec::new());
            },
            UpdateFilters(filters) =>  self.update_tasks(self.model.tag, &self.model.list, filters),
        }
    }

    view!
    {
        #[name="filter"]
        ::widgets::Filter {
            Complete(ref task) => Msg::Complete(task.clone()),
            Filters(ref filter) => Msg::UpdateFilters(filter.clone()),
        }
    }
}
