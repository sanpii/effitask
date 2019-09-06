use gtk::prelude::*;
use relm_attributes::widget;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Set(Option<todo_txt::task::Recurrence>),
    Updated(Option<todo_txt::task::Recurrence>),
    UpdateNum,
    UpdatePeriod,
    UpdateStrict,
}

impl Repeat {
    fn set_recurrence(&self, recurrence: Option<todo_txt::task::Recurrence>) {
        self.day.set_active(false);
        self.week.set_active(false);
        self.month.set_active(false);
        self.year.set_active(false);

        if let Some(recurrence) = recurrence {
            use todo_txt::task::Period::*;

            self.num.set_text(format!("{}", recurrence.num).as_str());
            self.strict.set_active(recurrence.strict);

            match recurrence.period {
                Day => self.day.set_active(true),
                Week => self.week.set_active(true),
                Month => self.month.set_active(true),
                Year => self.year.set_active(true),
            }
        } else {
            self.num.set_text("");
            self.strict.set_active(false);
        }
    }

    fn update_recurrence(&self) {
        let recurrence = self.get_recurrence();

        self.model.stream().emit(Msg::Updated(recurrence));
    }

    fn get_recurrence(&self) -> Option<todo_txt::task::Recurrence> {
        let num = self.num.get_value() as i64;

        if num == 0 {
            return None;
        }

        let strict = self.strict.get_active();

        let period = if self.day.get_active() {
            todo_txt::task::Period::Day
        } else if self.week.get_active() {
            todo_txt::task::Period::Week
        } else if self.month.get_active() {
            todo_txt::task::Period::Month
        } else if self.year.get_active() {
            todo_txt::task::Period::Year
        } else {
            return None;
        };

        Some(todo_txt::task::Recurrence {
            num,
            period,
            strict,
        })
    }
}

#[widget]
impl relm::Widget for Repeat {
    fn init_view(&mut self) {
        self.num.set_adjustment(&gtk::Adjustment::new(
            0.,
            0.,
            std::usize::MAX as f64,
            1.,
            5.,
            1.,
        ));
        self.set_recurrence(None);

        self.week.join_group(Some(&self.day));
        self.month.join_group(Some(&self.day));
        self.year.join_group(Some(&self.day));
    }

    fn model(relm: &relm::Relm<Self>, _: ()) -> relm::Relm<Repeat> {
        relm.clone()
    }

    fn update(&mut self, event: Msg) {
        use self::Msg::*;

        match event {
            Set(recurrence) => self.set_recurrence(recurrence),
            Updated(_) => (),
            UpdateNum => self.update_recurrence(),
            UpdatePeriod => self.update_recurrence(),
            UpdateStrict => self.update_recurrence(),
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            #[name="num"]
            gtk::SpinButton {
                focus_out_event(_, _) => (Msg::UpdateNum, gtk::Inhibit(false)),
            },
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,
                #[name="day"]
                gtk::RadioButton {
                    label: "d",
                    tooltip_text: Some("Day"),
                    mode: false,
                    toggled => Msg::UpdatePeriod,
                },
                #[name="week"]
                gtk::RadioButton {
                    label: "w",
                    tooltip_text: Some("Week"),
                    mode: false,
                    toggled => Msg::UpdatePeriod,
                },
                #[name="month"]
                gtk::RadioButton {
                    label: "m",
                    tooltip_text: Some("Month"),
                    mode: false,
                    toggled => Msg::UpdatePeriod,
                },
                #[name="year"]
                gtk::RadioButton {
                    label: "y",
                    tooltip_text: Some("Year"),
                    mode: false,
                    toggled => Msg::UpdatePeriod,
                },
                #[name="strict"]
                gtk::CheckButton {
                    child: {
                        expand: true,
                    },
                    halign: gtk::Align::Center,
                    label: "Strict",
                    tooltip_text: Some("Use real due date as offset, not today"),
                    toggled => Msg::UpdateStrict,
                },
            },
        }
    }
}
