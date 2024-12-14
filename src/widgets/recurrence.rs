use gtk::prelude::*;

#[derive(Debug)]
pub enum MsgInput {
    Update,
}

#[derive(Debug)]
pub enum MsgOutput {
    Updated(Option<todo_txt::task::Recurrence>),
}

#[derive(Default)]
pub struct Model {
    num: relm4::binding::F64Binding,
    day: relm4::binding::BoolBinding,
    week: relm4::binding::BoolBinding,
    month: relm4::binding::BoolBinding,
    year: relm4::binding::BoolBinding,
    strict: relm4::binding::BoolBinding,
}

impl Model {
    fn recurrence(&self) -> Option<todo_txt::task::Recurrence> {
        let num = self.num.value() as i64;

        if num == 0 {
            return None;
        }

        let period = if self.day.value() {
            todo_txt::task::Period::Day
        } else if self.week.value() {
            todo_txt::task::Period::Week
        } else if self.month.value() {
            todo_txt::task::Period::Month
        } else if self.year.value() {
            todo_txt::task::Period::Year
        } else {
            return None;
        };

        Some(todo_txt::task::Recurrence {
            num,
            period,
            strict: self.strict.value(),
        })
    }

    fn set(&mut self, recurrence: Option<todo_txt::task::Recurrence>) {
        self.num.set_value(
            recurrence
                .as_ref()
                .map(|x| x.num as f64)
                .unwrap_or_default(),
        );

        match recurrence.as_ref().map(|x| x.period) {
            Some(todo_txt::task::Period::Day) => self.day.set_value(true),
            Some(todo_txt::task::Period::Week) => self.week.set_value(true),
            Some(todo_txt::task::Period::Month) => self.month.set_value(true),
            Some(todo_txt::task::Period::Year) => self.year.set_value(true),
            None => (),
        };

        self.strict
            .set_value(recurrence.as_ref().map(|x| x.strict).unwrap_or_default());
    }
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Model {
    type Init = Option<todo_txt::task::Recurrence>;
    type Input = MsgInput;
    type Output = MsgOutput;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        use relm4::binding::ConnectBindingExt as _;

        let mut model = Self::default();
        model.set(init);

        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: relm4::ComponentSender<Self>) {
        use MsgInput::*;

        match msg {
            Update => {
                sender.output(MsgOutput::Updated(self.recurrence())).ok();
            }
        }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::SpinButton::with_binding(&model.num) {
                set_adjustment: &gtk::Adjustment::new(0., 0., usize::MAX as f64, 1., 5., 1.),

                connect_value_changed => MsgInput::Update,
            },
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                append: group = &gtk::ToggleButton::with_binding(&model.day) {
                    set_label: "d",
                    set_tooltip_text: Some("Day"),

                    connect_toggled => MsgInput::Update,
                },
                gtk::ToggleButton::with_binding(&model.week) {
                    set_label: "w",
                    set_tooltip_text: Some("Week"),
                    set_group: Some(&group),

                    connect_toggled => MsgInput::Update,
                },
                gtk::ToggleButton::with_binding(&model.month) {
                    set_label: "m",
                    set_tooltip_text: Some("Month"),
                    set_group: Some(&group),

                    connect_toggled => MsgInput::Update,
                },
                gtk::ToggleButton::with_binding(&model.year) {
                    set_label: "y",
                    set_tooltip_text: Some("Year"),
                    set_group: Some(&group),

                    connect_toggled => MsgInput::Update,
                },
                gtk::CheckButton::with_binding(&model.strict) {
                    set_halign: gtk::Align::Center,
                    set_hexpand: true,
                    set_label: Some("Strict"),
                    set_tooltip_text: Some("Use real due date as offset, not today"),

                    connect_toggled => MsgInput::Update,
                },
            },
        }
    }
}
