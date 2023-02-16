use crate::widgets::filter::Msg::{Complete, Edit, Filters};
use crate::widgets::Filter;

#[derive(Clone, Copy)]
pub enum Type {
    Projects,
    Contexts,
}

#[derive(relm_derive::Msg)]
pub enum Msg {
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    UpdateFilters(Vec<String>),
    Update,
}

impl Tags {
    fn update_tags(&self, tag: Type) {
        let list = crate::application::tasks();
        let tags = match tag {
            Type::Projects => list.projects(),
            Type::Contexts => list.contexts(),
        };

        let tags = tags
            .iter()
            .map(|x| (x.clone(), self.get_progress(tag, &list, x)))
            .filter(|&(_, (done, total))| done != total)
            .collect();

        self.components
            .filter
            .emit(crate::widgets::filter::Msg::UpdateFilters(tags));
    }

    fn get_progress(&self, tag: Type, list: &crate::tasks::List, current: &str) -> (u32, u32) {
        list.tasks
            .iter()
            .filter(|x| {
                for tag in self.get_tags(tag, x) {
                    if tag == current || tag.starts_with(format!("{current}-").as_str()) {
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

    fn update_tasks(&self, tag: Type, filters: &[String]) {
        let today = crate::date::today();
        let preferences = crate::application::preferences();
        let list = crate::application::tasks();

        let tasks = list
            .tasks
            .iter()
            .filter(|x| {
                let tags = self.get_tags(tag, x);

                (preferences.done || !x.finished)
                    && !tags.is_empty()
                    && Self::has_filter(tags, filters)
                    && (preferences.defered
                        || x.threshold_date.is_none()
                        || x.threshold_date.unwrap() <= today)
            })
            .cloned()
            .collect();

        self.components
            .filter
            .emit(crate::widgets::filter::Msg::UpdateTasks(tasks));
    }

    fn get_tags<'a>(&self, tag: Type, task: &'a crate::tasks::Task) -> &'a Vec<String> {
        match tag {
            Type::Projects => &task.projects,
            Type::Contexts => &task.contexts,
        }
    }

    fn has_filter(tags: &[String], filters: &[String]) -> bool {
        for filter in filters {
            if tags.contains(filter) {
                return true;
            }
        }

        false
    }
}

#[relm_derive::widget]
impl relm::Widget for Tags {
    fn model(tag: Type) -> Type {
        tag
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Complete(_) | Edit(_) => (),
            Update => {
                self.update_tags(self.model);
                self.update_tasks(self.model, &[]);
            }
            UpdateFilters(filters) => self.update_tasks(self.model, &filters),
        }
    }

    view! {
        #[name="filter"]
        Filter {
            Complete(ref task) => Msg::Complete(task.clone()),
            Edit(ref task) => Msg::Edit(task.clone()),
            Filters(ref filter) => Msg::UpdateFilters(filter.clone()),
        }
    }
}
