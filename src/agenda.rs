use gtk;
use gtk::prelude::*;
use relm_attributes::widget;
use widgets::tasks::Msg::{Complete, Edit};

#[derive(Msg)]
pub enum Msg {
    Complete(Box<::tasks::Task>),
    Edit(Box<::tasks::Task>),
    Selected,
    Select(::chrono::DateTime<::chrono::Local>),
    Update(::tasks::List, bool, bool),
}

pub struct Model {
    list: ::tasks::List,
    defered: bool,
    done: bool,
}

macro_rules! update {
    ($self:ident, $exp:ident, $task:ident, $get:ident, $date:ident) => {
        let tasks = $self.$get($date);

        $self.$exp.set_expanded(!tasks.is_empty());
        $self.$exp.set_sensitive(!tasks.is_empty());
        $self.$task.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

impl Widget
{
    fn update_tasks(&self)
    {
        self.calendar.clear_marks();

        let (y, m, d) = self.calendar.get_date();
        let date = ::chrono::naive::NaiveDate::from_ymd(y as i32, m + 1, d);

        update!(self, past_exp, past, get_past_tasks, date);
        update!(self, today_exp, today, get_today_tasks, date);
        update!(self, tomorrow_exp, tomorrow, get_tomorrow_tasks, date);
        update!(self, week_exp, week, get_week_tasks, date);
        update!(self, month_exp, month, get_month_tasks, date);
    }

    fn get_past_tasks(&self, date: ::chrono::naive::NaiveDate) -> Vec<::tasks::Task>
    {
        self.get_tasks(None, Some(date))
    }

    fn get_today_tasks(&self, date: ::chrono::naive::NaiveDate) -> Vec<::tasks::Task>
    {
        self.get_tasks(Some(date), Some(date.succ()))
    }

    fn get_tomorrow_tasks(&self, date: ::chrono::naive::NaiveDate) -> Vec<::tasks::Task>
    {
        self.get_tasks(Some(date.succ()), Some(date + ::chrono::Duration::days(2)))
    }

    fn get_week_tasks(&self, date: ::chrono::naive::NaiveDate) -> Vec<::tasks::Task>
    {
        self.get_tasks(Some(date + ::chrono::Duration::days(2)), Some(date + ::chrono::Duration::weeks(1)))
    }

    fn get_month_tasks(&self, date: ::chrono::naive::NaiveDate) -> Vec<::tasks::Task>
    {
        self.get_tasks(Some(date + ::chrono::Duration::weeks(1)), Some(date + ::chrono::Duration::weeks(4)))
    }

    fn get_tasks(&self, start: Option<::chrono::naive::NaiveDate>, end: Option<::chrono::naive::NaiveDate>) -> Vec<::tasks::Task>
    {
        let (_, month, _) = self.calendar.get_date();

        let tasks: Vec<::tasks::Task> = self.model.list.tasks.iter()
            .filter(|x| {
                if let Some(due_date) = x.due_date {
                    (self.model.done || !x.finished)
                        && (self.model.defered || x.threshold_date.is_none() || start.is_none() || x.threshold_date.unwrap() <= start.unwrap())
                        && (start.is_none() || due_date >= start.unwrap())
                        && (end.is_none() || due_date < end.unwrap())
                }
                else {
                    false
                }
            })
            .map(|x| {
                use chrono::Datelike;

                let due_date = x.due_date.unwrap();

                if due_date.month0() == month {
                    self.calendar.mark_day(due_date.day());
                }

                x.clone()
            })
            .collect();

        tasks.clone()
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn model(_: ()) -> Model
    {
        Model {
            list: ::tasks::List::new(),
            defered: false,
            done: false,
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Selected => self.update_tasks(),
            Select(date) => {
                use chrono::Datelike;

                self.calendar.select_month(date.month0(), date.year() as u32);
                self.calendar.select_day(date.day());
            },
            Update(list, defered, done) => {
                self.model = Model {
                    list: list.clone(),
                    defered: defered,
                    done: done,
                };
                self.update_tasks();
            },
        }
    }

    view!
    {
        gtk::Box {
            orientation: ::gtk::Orientation::Horizontal,
            spacing: 10,
            gtk::Box {
                orientation: ::gtk::Orientation::Vertical,
                #[name="calendar"]
                gtk::Calendar {
                    day_selected => Msg::Selected,
                },
                gtk::Button {
                    packing: {
                        padding: 5,
                    },
                    label: "Today",
                    clicked => Msg::Select(::chrono::Local::now()),
                },
            },
            gtk::ScrolledWindow {
                packing: {
                    expand: true,
                },
                gtk::Box {
                    orientation: ::gtk::Orientation::Vertical,
                    #[name="past_exp"]
                    gtk::Expander {
                        label: "Past due",
                        #[name="past"]
                        ::widgets::Tasks {
                            property_vscrollbar_policy: ::gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        },
                    },
                    #[name="today_exp"]
                    gtk::Expander {
                        label: "Today",
                        #[name="today"]
                        ::widgets::Tasks {
                            property_vscrollbar_policy: ::gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        },
                    },
                    #[name="tomorrow_exp"]
                    gtk::Expander {
                        label: "Tomorrow",
                        #[name="tomorrow"]
                        ::widgets::Tasks {
                            property_vscrollbar_policy: ::gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        },
                    },
                    #[name="week_exp"]
                    gtk::Expander {
                        label: "This week",
                        #[name="week"]
                        ::widgets::Tasks {
                            property_vscrollbar_policy: ::gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        },
                    },
                    #[name="month_exp"]
                    gtk::Expander {
                        label: "This month",
                        #[name="month"]
                        ::widgets::Tasks {
                            property_vscrollbar_policy: ::gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        }
                    },
                },
            },
        }
    }
}
