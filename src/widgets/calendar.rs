use gtk::prelude::*;

pub struct Model {
    label: String,
    popover: gtk::Popover,
    calendar: gtk::Calendar,
    relm: relm::Relm<Calendar>,
}

#[derive(relm_derive::Msg)]
pub enum Msg {
    Add(todo_txt::task::Period),
    DateSelected,
    DateUpdated,
    Sensitive,
    Set(Option<chrono::NaiveDate>),
    ShowCalendar,
    Updated(Option<chrono::NaiveDate>),
}

impl Calendar {
    fn add(&self, period: todo_txt::task::Period) {
        let mut date = crate::date::today();

        let text = self.widgets.entry.get_text();

        if !text.is_empty() {
            date = match chrono::NaiveDate::parse_from_str(text.as_str(), "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    log::error!("Invalid date format, use YYYY-MM-DD");
                    return;
                }
            };
        }

        date = period + date;
        self.set_date(Some(date));
        self.date_updated();
    }

    fn date_selected(&self) {
        let (y, m, d) = self.model.calendar.get_date();

        self.widgets
            .entry
            .set_text(format!("{}-{}-{}", y, m + 1, d).as_str());
        self.model.popover.popdown();

        self.date_updated();
    }

    fn date_updated(&self) {
        let mut date = None;
        let text = self.widgets.entry.get_text();

        if !text.is_empty() {
            date = match chrono::NaiveDate::parse_from_str(text.as_str(), "%Y-%m-%d") {
                Ok(date) => Some(date),
                Err(_) => {
                    log::error!("Invalid date format, use YYYY-MM-DD");
                    return;
                }
            };
        }

        self.model.relm.stream().emit(Msg::Updated(date));
    }

    fn set_date(&self, date: Option<chrono::NaiveDate>) {
        if let Some(date) = date {
            use chrono::Datelike;

            self.widgets
                .entry
                .set_text(date.format("%Y-%m-%d").to_string().as_str());
            self.model
                .calendar
                .select_month(date.month() - 1, date.year() as u32);
            self.model.calendar.select_day(date.day());
        } else {
            self.widgets.entry.set_text("");
        }
    }

    fn sensitive(&self) {
        use relm::Widget;

        if self.root().get_sensitive() {
            self.widgets.buttons.show();
        } else {
            self.widgets.buttons.hide();
        }
    }
}

#[relm_derive::widget]
impl relm::Widget for Calendar {
    fn init_view(&mut self) {
        self.widgets
            .entry
            .set_icon_from_icon_name(gtk::EntryIconPosition::Primary, Some("x-office-calendar"));

        self.widgets.label.set_size_request(200, -1);
        self.widgets.label.set_text(self.model.label.as_str());

        relm::connect!(
            self.model.relm,
            self.model.calendar,
            connect_day_selected(_),
            Msg::DateSelected
        );
        self.model.calendar.show();
        self.model
            .popover
            .set_relative_to(Some(&self.widgets.entry));
        self.model.popover.set_pointing_to(&gdk::Rectangle {
            x: 15,
            y: 15,
            width: 0,
            height: 0,
        });
        self.model.popover.add(&self.model.calendar);
        self.model.popover.hide();
    }

    fn model(relm: &relm::Relm<Self>, label: String) -> Model {
        Model {
            label,
            popover: gtk::Popover::new(None::<&gtk::Calendar>),
            calendar: gtk::Calendar::new(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Add(period) => self.add(period),
            DateSelected => self.date_selected(),
            DateUpdated => self.date_updated(),
            Sensitive => self.sensitive(),
            Set(date) => self.set_date(date),
            ShowCalendar => self.model.popover.popup(),
            Updated(_) => (),
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Horizontal,
            spacing: 10,
            property_sensitive_notify => Msg::Sensitive,

            #[name="label"]
            gtk::Label {
                child: {
                    expand: true,
                    fill: true,
                },
                xalign: 1.,
                yalign: 0.,
            },

            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                #[name="entry"]
                gtk::Entry {
                    child: {
                        expand: true,
                        fill: true,
                    },
                    property_width_request: 214,
                    focus_out_event(_, _) => (Msg::DateUpdated, gtk::Inhibit(false)),
                    icon_press(_, _, _) => Msg::ShowCalendar,
                },
                #[name="buttons"]
                gtk::Box {
                    orientation: gtk::Orientation::Horizontal,
                    gtk::Button {
                        child: {
                            pack_type: gtk::PackType::End,
                        },
                        label: "+1y",
                        tooltip_text: Some("Add one year"),
                        clicked => Msg::Add(todo_txt::task::Period::Year),
                    },
                    gtk::Button {
                        child: {
                            pack_type: gtk::PackType::End,
                        },
                        label: "+1m",
                        tooltip_text: Some("Add one month"),
                        clicked => Msg::Add(todo_txt::task::Period::Month),
                    },
                    gtk::Button {
                        child: {
                            pack_type: gtk::PackType::End,
                        },
                        label: "+1w",
                        tooltip_text: Some("Add one month"),
                        clicked => Msg::Add(todo_txt::task::Period::Week),
                    },
                    gtk::Button {
                        child: {
                            pack_type: gtk::PackType::End,
                        },
                        label: "+1d",
                        tooltip_text: Some("Add one month"),
                        clicked => Msg::Add(todo_txt::task::Period::Day),
                    },
                },
            },
        },
    }
}
