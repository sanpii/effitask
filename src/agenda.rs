use chrono::Datelike as _;
use gtk::prelude::*;

#[derive(Debug)]
pub enum MsgInput {
    CalendarChange(Change),
    DateSelect(chrono::NaiveDate),
    Update,
}

#[derive(Debug)]
pub enum Change {
    PrevMonth,
    PrevYear,
    NextMonth,
    NextYear,
}

#[derive(Debug)]
pub enum MsgOutput {
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
}

macro_rules! create {
    ($sender:ident) => {{
        let component = crate::widgets::tasks::Model::builder().launch(()).forward(
            $sender.output_sender(),
            |output| match output {
                crate::widgets::task::MsgOutput::Complete(task) => MsgOutput::Complete(task),
                crate::widgets::task::MsgOutput::Edit(task) => MsgOutput::Edit(task),
            },
        );
        component
            .widget()
            .set_vscrollbar_policy(gtk::PolicyType::Never);

        component
    }};
}

macro_rules! update {
    ($self:ident, $exp:ident, $task:ident, $get:ident, $list:ident, $date:ident) => {{
        use relm4::ComponentController as _;

        let tasks = $self.$get(&$list, $date);

        $self.$exp.set_expanded(!tasks.is_empty());
        $self.$exp.set_sensitive(!tasks.is_empty());
        $self.$task.emit(crate::widgets::tasks::Msg::Update(tasks));
    }};
}

pub struct Model {
    calendar: gtk::Calendar,
    date: chrono::NaiveDate,
    month_exp: gtk::Expander,
    month: relm4::Controller<crate::widgets::tasks::Model>,
    past_exp: gtk::Expander,
    past: relm4::Controller<crate::widgets::tasks::Model>,
    today_exp: gtk::Expander,
    today: relm4::Controller<crate::widgets::tasks::Model>,
    tomorrow_exp: gtk::Expander,
    tomorrow: relm4::Controller<crate::widgets::tasks::Model>,
    week_exp: gtk::Expander,
    week: relm4::Controller<crate::widgets::tasks::Model>,
}

impl Model {
    fn update_tasks(&self) {
        let list = crate::application::tasks();
        let date = crate::date::from_glib(self.calendar.date());

        update!(self, past_exp, past, past_tasks, list, date);
        update!(self, today_exp, today, today_tasks, list, date);
        update!(self, tomorrow_exp, tomorrow, tomorrow_tasks, list, date);
        update!(self, week_exp, week, week_tasks, list, date);
        update!(self, month_exp, month, month_tasks, list, date);
    }

    fn past_tasks(
        &self,
        list: &crate::tasks::List,
        date: chrono::naive::NaiveDate,
    ) -> Vec<crate::tasks::Task> {
        self.tasks(list, None, Some(date))
    }

    fn today_tasks(
        &self,
        list: &crate::tasks::List,
        date: chrono::naive::NaiveDate,
    ) -> Vec<crate::tasks::Task> {
        self.tasks(list, Some(date), Some(date + chrono::Duration::days(1)))
    }

    fn tomorrow_tasks(
        &self,
        list: &crate::tasks::List,
        date: chrono::naive::NaiveDate,
    ) -> Vec<crate::tasks::Task> {
        self.tasks(
            list,
            Some(date + chrono::Duration::days(1)),
            Some(date + chrono::Duration::days(2)),
        )
    }

    fn week_tasks(
        &self,
        list: &crate::tasks::List,
        date: chrono::naive::NaiveDate,
    ) -> Vec<crate::tasks::Task> {
        self.tasks(
            list,
            Some(date + chrono::Duration::days(2)),
            Some(date + chrono::Duration::weeks(1)),
        )
    }

    fn month_tasks(
        &self,
        list: &crate::tasks::List,
        date: chrono::naive::NaiveDate,
    ) -> Vec<crate::tasks::Task> {
        self.tasks(
            list,
            Some(date + chrono::Duration::weeks(1)),
            Some(date + chrono::Duration::weeks(4)),
        )
    }

    fn tasks(
        &self,
        list: &crate::tasks::List,
        start: Option<chrono::naive::NaiveDate>,
        end: Option<chrono::naive::NaiveDate>,
    ) -> Vec<crate::tasks::Task> {
        let preferences = crate::application::preferences();

        let tasks = list
            .tasks
            .iter()
            .filter(|x| {
                if let Some(due_date) = x.due_date {
                    (preferences.done || !x.finished)
                        && (preferences.defered
                            || x.threshold_date.is_none()
                            || start.is_none()
                            || x.threshold_date.unwrap() <= start.unwrap())
                        && (start.is_none() || due_date >= start.unwrap())
                        && (end.is_none() || due_date < end.unwrap())
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        tasks
    }

    fn update_marks(&self) {
        use chrono::Datelike as _;

        self.calendar.clear_marks();

        let list = crate::application::tasks();
        let date = self.calendar.date();
        let month = date.month() as u32;
        let year = date.year();

        for task in &list.tasks {
            let Some(due_date) = task.due_date else {
                continue;
            };

            if due_date.year() == year && due_date.month() == month {
                self.calendar.mark_day(due_date.day());
            }
        }
    }
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Model {
    type Init = chrono::NaiveDate;
    type Input = MsgInput;
    type Output = MsgOutput;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        use relm4::Component as _;
        use relm4::ComponentController as _;

        let model = Self {
            calendar: gtk::Calendar::new(),
            date: init,
            month: create!(sender),
            month_exp: gtk::Expander::new(None),
            past: create!(sender),
            past_exp: gtk::Expander::new(None),
            today: create!(sender),
            today_exp: gtk::Expander::new(None),
            tomorrow: create!(sender),
            tomorrow_exp: gtk::Expander::new(None),
            week: create!(sender),
            week_exp: gtk::Expander::new(None),
        };

        let calendar = &model.calendar;
        let month_exp = &model.month_exp;
        let past_exp = &model.past_exp;
        let today_exp = &model.today_exp;
        let tomorrow_exp = &model.tomorrow_exp;
        let week_exp = &model.week_exp;
        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: relm4::ComponentSender<Self>) {
        use MsgInput::*;

        match msg {
            CalendarChange(change) => {
                self.date = match change {
                    Change::NextMonth => self.date.checked_add_months(chrono::Months::new(1)),
                    Change::NextYear => self.date.checked_add_months(chrono::Months::new(12)),
                    Change::PrevMonth => self.date.checked_sub_months(chrono::Months::new(1)),
                    Change::PrevYear => self.date.checked_sub_months(chrono::Months::new(12)),
                }
                .unwrap();

                self.update_marks();
            }
            DateSelect(date) => self.date = date,
            Update => (),
        }

        self.update_tasks();
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                #[local_ref]
                calendar -> gtk::Calendar {
                    #[watch]
                    set_day: model.date.day() as i32,
                    #[watch]
                    set_month: model.date.month() as i32 - 1,
                    #[watch]
                    set_year: model.date.year(),

                    connect_day_selected[sender] => move |this| {
                        sender.input(MsgInput::DateSelect(crate::date::from_glib(this.date())));
                    },
                    connect_next_month => MsgInput::CalendarChange(Change::NextMonth),
                    connect_next_year => MsgInput::CalendarChange(Change::NextYear),
                    connect_prev_month => MsgInput::CalendarChange(Change::PrevMonth),
                    connect_prev_year => MsgInput::CalendarChange(Change::PrevYear),
                },
                gtk::Button {
                    set_label: "Today",
                    connect_clicked => MsgInput::DateSelect(crate::date::today()),
                },
            },
            gtk::ScrolledWindow {
                gtk::Box {
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,

                    #[local_ref]
                    past_exp -> gtk::Expander {
                        set_child: Some(model.past.widget()),
                        set_label: Some("Past due"),
                    },
                    #[local_ref]
                    today_exp -> gtk::Expander {
                        set_child: Some(model.today.widget()),
                        set_label: Some("Today"),
                    },
                    #[local_ref]
                    tomorrow_exp -> gtk::Expander {
                        set_child: Some(model.tomorrow.widget()),
                        set_label: Some("Tomorrow"),
                    },
                    #[local_ref]
                    week_exp -> gtk::Expander {
                        set_child: Some(model.week.widget()),
                        set_label: Some("This week"),
                    },
                    #[local_ref]
                    month_exp -> gtk::Expander {
                        set_child: Some(model.month.widget()),
                        set_label: Some("This month"),
                    },
                },
            },
        }
    }
}
