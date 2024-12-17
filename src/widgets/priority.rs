use gtk::prelude::*;

#[derive(Debug)]
pub enum MsgInput {
    More,
}

#[derive(Debug)]
pub enum MsgOutput {
    Updated(todo_txt::Priority),
}

pub struct Model {
    priority: todo_txt::Priority,
    show_more: bool,
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Model {
    type Init = todo_txt::Priority;
    type Input = MsgInput;
    type Output = MsgOutput;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Self {
            priority: init,
            show_more: false,
        };

        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: relm4::ComponentSender<Self>) {
        match msg {
            MsgInput::More => self.show_more = true,
        }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                #[watch]
                set_visible: !model.show_more,

                append: group = &gtk::ToggleButton {
                    set_active: model.priority == 0,
                    set_label: "A",

                    connect_toggled[sender] => move |_| {
                        sender.output(MsgOutput::Updated(0.into())).ok();
                    },
                },
                #[name="b"]
                gtk::ToggleButton {
                    set_active: model.priority == 1,
                    set_group: Some(&group),
                    set_label: "B",

                    connect_toggled[sender] => move |_| {
                        sender.output(MsgOutput::Updated(1.into())).ok();
                    },
                },
                #[name="c"]
                gtk::ToggleButton {
                    set_active: model.priority == 2,
                    set_group: Some(&group),
                    set_label: "C",

                    connect_toggled[sender] => move |_| {
                        sender.output(MsgOutput::Updated(2.into())).ok();
                    },
                },
                #[name="d"]
                gtk::ToggleButton {
                    set_active: model.priority == 3,
                    set_group: Some(&group),
                    set_label: "D",

                    connect_toggled[sender] => move |_| {
                        sender.output(MsgOutput::Updated(3.into())).ok();
                    },
                },
                #[name="e"]
                gtk::ToggleButton {
                    set_active: model.priority == 4,
                    set_group: Some(&group),
                    set_label: "E",

                    connect_toggled[sender] => move |_| {
                        sender.output(MsgOutput::Updated(4.into())).ok();
                    },
                },
                gtk::Button {
                    set_label: "…",
                    set_tooltip_text: Some("More"),

                    connect_clicked => MsgInput::More,
                },
                #[name="z"]
                gtk::ToggleButton {
                    set_active: model.priority == 26,
                    set_group: Some(&group),
                    set_label: "Z",

                    connect_clicked[sender] => move |_| {
                        sender.output(MsgOutput::Updated(26.into())).ok();
                    },
                },
            },
            gtk::SpinButton {
                set_adjustment: &gtk::Adjustment::new(0., 0., 27., 1., 5., 1.),
                set_climb_rate: 1.,
                set_digits: 0,
                #[watch]
                set_visible: model.show_more,

                connect_value_changed[sender] => move |button| {
                    let priority = (button.value() as u8).into();
                    sender.output(MsgOutput::Updated(priority)).ok();
                },
            },
        },
    }
}
