use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Selected,
    Select(::chrono::DateTime<::chrono::Local>),
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
    fn populate(&mut self)
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

        let tasks: Vec<::tasks::Task> = self.model.tasks.iter()
            .filter(|x| {
                if let Some(due_date) = x.due_date {
                    !x.finished
                        && (x.threshold_date.is_none() || start.is_none() || x.threshold_date.unwrap() <= start.unwrap())
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
    fn init_view(&mut self)
    {
        self.populate();
    }

    fn model(tasks: ::tasks::List) -> ::tasks::List
    {
        tasks
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Selected => self.populate(),
            Select(date) => {
                use chrono::Datelike;

                self.calendar.select_month(date.month0(), date.year() as u32);
                self.calendar.select_day(date.day());
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
                        ::widgets::Tasks,
                    },
                    #[name="today_exp"]
                    gtk::Expander {
                        label: "Today",
                        #[name="today"]
                        ::widgets::Tasks,
                    },
                    #[name="tomorrow_exp"]
                    gtk::Expander {
                        label: "Tomorrow",
                        #[name="tomorrow"]
                        ::widgets::Tasks,
                    },
                    #[name="week_exp"]
                    gtk::Expander {
                        label: "This week",
                        #[name="week"]
                        ::widgets::Tasks,
                    },
                    #[name="month_exp"]
                    gtk::Expander {
                        label: "This month",
                        #[name="month"]
                        ::widgets::Tasks {
                        }
                    },
                },
            },
        }
    }
}
