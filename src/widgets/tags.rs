use relm_attributes::widget;
use widgets::filter::Msg::{Complete, Edit, Filters};

#[derive(Clone, Copy)]
pub enum Type {
    Projects,
    Contexts,
}

#[derive(Msg)]
pub enum Msg {
    Complete(Box<::tasks::Task>),
    Edit(Box<::tasks::Task>),
    UpdateFilters(Vec<String>),
    Update(::tasks::List, bool, bool),
}

pub struct Model {
    defered: bool,
    done: bool,
    list: ::tasks::List,
    tag: Type,
}

impl Tags
{
    fn update_tags(&self, tag: Type)
    {
        let tags = match tag {
            Type::Projects => self.model.list.projects(),
            Type::Contexts => self.model.list.contexts(),
        };

        let tags = tags.iter()
            .map(|x| (x.clone(), self.get_progress(tag, &self.model.list, x)))
            .filter(|&(_, (done, total))| done != total)
            .collect();

        self.filter.emit(::widgets::filter::Msg::UpdateFilters(tags));
    }

    fn get_progress(&self, tag: Type, list: &::tasks::List, current: &str) -> (u32, u32)
    {
        list.tasks.iter()
            .filter(|x| {
                for tag in self.get_tags(tag, x) {
                    if tag == current || tag.starts_with(format!("{}-", current).as_str()) {
                        return true;
                    }
                }

                false
            })
            .fold((0, 0), |(mut done, total), x| {
                if x.finished {
                    done += 1;
                }

                (done, total + 1)
            })
    }

    fn update_tasks(&self, tag: Type, filters: &[String])
    {
        let today = ::date::today();

        let tasks = self.model.list.tasks.iter()
            .filter(|x| {
                let tags = self.get_tags(tag, x);

                (self.model.done || !x.finished)
                    && !tags.is_empty()
                    && Self::has_filter(tags, filters)
                    && (self.model.defered || x.threshold_date.is_none() || x.threshold_date.unwrap() <= today)
            })
            .cloned()
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

    fn has_filter(tags: &[String], filters: &[String]) -> bool
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
            defered: false,
            done: false,
            list: ::tasks::List::new(),
            tag: tag,
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Update(list, defered, done) =>  {
                self.model.list = list.clone();
                self.model.defered = defered;
                self.model.done = done;

                self.update_tags(self.model.tag);
                self.update_tasks(self.model.tag, &[]);
            },
            UpdateFilters(filters) =>  self.update_tasks(self.model.tag, &filters),
        }
    }

    view!
    {
        #[name="filter"]
        ::widgets::Filter {
            Complete(ref task) => Msg::Complete(task.clone()),
            Edit(ref task) => Msg::Edit(task.clone()),
            Filters(ref filter) => Msg::UpdateFilters(filter.clone()),
        }
    }
}
