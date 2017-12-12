use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

pub struct Model {
    label: String,
    popover: ::gtk::Popover,
    calendar: ::gtk::Calendar,
    relm: ::relm::Relm<Calendar>,
}

#[derive(Msg)]
pub enum Msg {
    DateSelected,
    DateUpdated,
    Set(Option<::chrono::NaiveDate>),
    Updated(Option<::chrono::NaiveDate>),
    ShowCalendar,
}

impl Calendar
{
    fn date_selected(&self)
    {
        let (y, m, d) = self.model.calendar.get_date();

        self.entry.set_text(format!("{}-{}-{}", y, m, d).as_str());
        self.model.popover.popdown();

        self.date_updated();
    }

    fn date_updated(&self)
    {
        let mut date = None;
        let text = self.entry.get_text()
            .unwrap();

        if !text.is_empty() {
            date = match ::chrono::NaiveDate::parse_from_str(text.as_str(), "%Y-%m-%d") {
                Ok(date) => Some(date),
                Err(_) => {
                    error!("Invalid date format, use YYYY-MM-DD");
                    return;
                },
            };
        }

        self.model.relm.stream()
            .emit(Msg::Updated(date));
    }

    fn set_date(&self, date: Option<::chrono::NaiveDate>)
    {
        if let Some(date) = date {
            self.entry.set_text(date.format("%Y-%m-%d").to_string().as_str());
        }
        else {
            self.entry.set_text("");
        }
    }
}

#[widget]
impl ::relm::Widget for Calendar
{
    fn init_view(&mut self)
    {
        self.entry.set_icon_from_icon_name(::gtk::EntryIconPosition::Primary, "x-office-calendar");

        self.label.set_size_request(200, -1);
        self.label.set_text(self.model.label.as_str());

        connect!(self.model.relm, self.model.calendar, connect_day_selected(_), Msg::DateSelected);
        self.model.calendar.show();
        self.model.popover.set_relative_to(&self.entry);
        self.model.popover.set_pointing_to(&::gdk::Rectangle {
            x: 15,
            y: 15,
            width: 0,
            height: 0,
        });
        self.model.popover.add(&self.model.calendar);
        self.model.popover.hide();
    }

    fn model(relm: &::relm::Relm<Self>, label: String) -> Model
    {
        Model {
            label: label.clone(),
            popover: ::gtk::Popover::new(None::<&::gtk::Calendar>),
            calendar: ::gtk::Calendar::new(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            DateSelected => self.date_selected(),
            DateUpdated => self.date_updated(),
            Set(date) => self.set_date(date),
            ShowCalendar => self.model.popover.popup(),
            Updated(_) => (),
        }
    }

    view!
    {
        gtk::Box {
            orientation: ::gtk::Orientation::Horizontal,
            spacing: 10,
            #[name="label"]
            gtk::Label {
                packing: {
                    expand: true,
                    fill: true,
                },
                xalign: 1.,
            },
            #[name="entry"]
            gtk::Entry {
                packing: {
                    expand: true,
                    fill: true,
                },
                focus_out_event(_, _) => (Msg::DateUpdated, ::gtk::Inhibit(false)),
                icon_press(_, _, _) => Msg::ShowCalendar,
            },
        },
    }
}
