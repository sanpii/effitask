use gtk::prelude::*;

#[derive(relm_derive::Msg)]
pub enum Msg {
    More,
    Set(u8),
    Updated(u8),
}

impl Priority {
    fn more(&self) {
        self.widgets.hbox.hide();
        self.widgets.button.show();
    }

    fn less(&self) {
        self.widgets.hbox.show();
        self.widgets.button.hide();
    }

    fn set(&self, priority: u8) {
        self.widgets.button.set_value(f64::from(priority));

        match priority {
            0 => self.widgets.a.set_active(true),
            1 => self.widgets.b.set_active(true),
            2 => self.widgets.c.set_active(true),
            3 => self.widgets.d.set_active(true),
            4 => self.widgets.e.set_active(true),
            26 => self.widgets.z.set_active(true),
            _ => (),
        }

        if priority < 5 || priority == 26 {
            self.less();
        } else {
            self.more();
        }
    }

    fn updated(&self, priority: u8) {
        self.widgets.button.set_value(f64::from(priority));
    }
}

#[relm_derive::widget]
impl relm::Widget for Priority {
    fn init_view(&mut self) {
        self.widgets
            .button
            .set_adjustment(&gtk::Adjustment::new(0., 0., 27., 1., 5., 1.));
        self.widgets.button.hide();

        self.widgets.b.join_group(Some(&self.widgets.a));
        self.widgets.c.join_group(Some(&self.widgets.a));
        self.widgets.d.join_group(Some(&self.widgets.a));
        self.widgets.e.join_group(Some(&self.widgets.a));
        self.widgets.z.join_group(Some(&self.widgets.a));
    }

    fn model(_: ()) {}

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            More => self.more(),
            Set(priority) => self.set(priority),
            Updated(priority) => self.updated(priority),
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            #[name="hbox"]
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,
                #[name="a"]
                gtk::RadioButton {
                    label: "A",
                    mode: false,
                    toggled => Msg::Updated(0),
                },
                #[name="b"]
                gtk::RadioButton {
                    label: "B",
                    mode: false,
                    toggled => Msg::Updated(1),
                },
                #[name="c"]
                gtk::RadioButton {
                    label: "C",
                    mode: false,
                    toggled => Msg::Updated(2),
                },
                #[name="d"]
                gtk::RadioButton {
                    label: "D",
                    mode: false,
                    toggled => Msg::Updated(3),
                },
                #[name="e"]
                gtk::RadioButton {
                    label: "E",
                    mode: false,
                    toggled => Msg::Updated(4),
                },
                gtk::Button {
                    label: "…",
                    tooltip_text: Some("More"),
                    clicked => Msg::More,
                },
                #[name="z"]
                gtk::RadioButton {
                    label: "Z",
                    mode: false,
                    clicked => Msg::Updated(26),
                },
            },
            #[name="button"]
            gtk::SpinButton {
                focus_out_event(button, _) => (
                    Msg::Updated(button.get_value() as u8),
                    gtk::Inhibit(false)
                ),
            },
        }
    }
}
