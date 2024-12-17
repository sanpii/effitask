use chrono::Datelike as _;
use gtk::prelude::*;

pub struct Model {
    date: Option<chrono::NaiveDate>,
    label: &'static str,
}

#[derive(Debug)]
pub enum MsgInput {
    Add(todo_txt::task::Period),
    DateSelected(gtk::glib::DateTime),
    DateUpdated,
    Set(Option<chrono::NaiveDate>),
}

#[derive(Debug)]
pub enum MsgOutput {
    Updated(Option<chrono::NaiveDate>),
}

impl Model {
    fn add(&mut self, sender: relm4::ComponentSender<Self>, period: todo_txt::task::Period) {
        self.date = Some(period + self.date.unwrap_or_else(crate::date::today));
        sender.output(MsgOutput::Updated(self.date)).ok();
    }

    fn date_selected(
        &mut self,
        widgets: &ModelWidgets,
        sender: relm4::ComponentSender<Self>,
        date: gtk::glib::DateTime,
    ) {
        self.date = Some(crate::date::from_glib(date));

        sender.output(MsgOutput::Updated(self.date)).ok();
        widgets.popover.popdown();
    }
}

#[relm4::component(pub)]
impl relm4::Component for Model {
    type CommandOutput = ();
    type Init = &'static str;
    type Input = MsgInput;
    type Output = MsgOutput;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Self {
            date: None,
            label: init,
        };

        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _: &Self::Root,
    ) {
        use MsgInput::*;

        match msg {
            Add(period) => self.add(sender, period),
            DateSelected(date) => self.date_selected(widgets, sender, date),
            Set(date) => self.date = date,
            DateUpdated => {
                sender.output(MsgOutput::Updated(self.date)).ok();
            }
        }
    }

    view! {
        #[name = "r#box"]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            gtk::Label {
                set_hexpand: true,
                set_text: &model.label,
                set_width_request: 200,
                set_xalign: 1.,
                set_yalign: 0.,
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                gtk::Box {
                    gtk::MenuButton {
                        set_icon_name: "x-office-calendar",
                        #[wrap(Some)]
                        #[name = "popover"]
                        set_popover = &gtk::Popover {
                            gtk::Calendar {
                                #[watch]
                                set_day?: model.date.map(|x| x.day() as i32),
                                #[watch]
                                set_month?: model.date.map(|x| x.month() as i32 - 1),
                                #[watch]
                                set_year?: model.date.map(|x| x.year()),

                                connect_day_selected[sender] => move |this| {
                                    sender.input(MsgInput::DateSelected(this.date()));
                                },
                            },
                        },
                    },
                    gtk::Entry {
                        set_hexpand: true,
                        #[watch]
                        set_text?: &model.date.map(|x| x.format("%Y-%m-%d").to_string()),
                        set_width_request: 214,

                        connect_move_focus[sender] => move |_, _| {
                            sender.input(MsgInput::DateUpdated);
                        },
                    },
                },
                gtk::Box {
                    set_halign: gtk::Align::End,
                    set_orientation: gtk::Orientation::Horizontal,
                    #[watch]
                    set_visible: r#box.is_sensitive(),

                    gtk::Button {
                        set_label: "+1y",
                        set_tooltip_text: Some("Add one year"),

                        connect_clicked => MsgInput::Add(todo_txt::task::Period::Year),
                    },
                    gtk::Button {
                        set_label: "+1m",
                        set_tooltip_text: Some("Add one month"),

                        connect_clicked => MsgInput::Add(todo_txt::task::Period::Month),
                    },
                    gtk::Button {
                        set_label: "+1w",
                        set_tooltip_text: Some("Add one month"),

                        connect_clicked => MsgInput::Add(todo_txt::task::Period::Week),
                    },
                    gtk::Button {
                        set_label: "+1d",
                        set_tooltip_text: Some("Add one month"),

                        connect_clicked => MsgInput::Add(todo_txt::task::Period::Day),
                    },
                },
            },
        },
    }
}
