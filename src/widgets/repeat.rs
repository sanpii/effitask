use gtk::prelude::*;

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
        self.widgets.day.set_active(false);
        self.widgets.week.set_active(false);
        self.widgets.month.set_active(false);
        self.widgets.year.set_active(false);

        if let Some(recurrence) = recurrence {
            use todo_txt::task::Period::*;

            self.widgets
                .num
                .set_text(format!("{}", recurrence.num).as_str());
            self.widgets.strict.set_active(recurrence.strict);

            match recurrence.period {
                Day => self.widgets.day.set_active(true),
                Week => self.widgets.week.set_active(true),
                Month => self.widgets.month.set_active(true),
                Year => self.widgets.year.set_active(true),
            }
        } else {
            self.widgets.num.set_text("");
            self.widgets.strict.set_active(false);
        }
    }

    fn update_recurrence(&self) {
        let recurrence = self.get_recurrence();

        self.model.stream().emit(Msg::Updated(recurrence));
    }

    fn get_recurrence(&self) -> Option<todo_txt::task::Recurrence> {
        let num = self.widgets.num.value() as i64;

        if num == 0 {
            return None;
        }

        let strict = self.widgets.strict.is_active();

        let period = if self.widgets.day.is_active() {
            todo_txt::task::Period::Day
        } else if self.widgets.week.is_active() {
            todo_txt::task::Period::Week
        } else if self.widgets.month.is_active() {
            todo_txt::task::Period::Month
        } else if self.widgets.year.is_active() {
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

#[relm_derive::widget]
impl relm::Widget for Repeat {
    fn init_view(&mut self) {
        self.widgets.num.set_adjustment(&gtk::Adjustment::new(
            0.,
            0.,
            usize::MAX as f64,
            1.,
            5.,
            1.,
        ));
        self.set_recurrence(None);

        self.widgets.week.join_group(Some(&self.widgets.day));
        self.widgets.month.join_group(Some(&self.widgets.day));
        self.widgets.year.join_group(Some(&self.widgets.day));
    }

    fn model(relm: &relm::Relm<Self>, _: ()) -> relm::Relm<Repeat> {
        relm.clone()
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Set(recurrence) => self.set_recurrence(recurrence),
            Updated(_) => (),
            UpdateNum | UpdatePeriod | UpdateStrict => self.update_recurrence(),
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
