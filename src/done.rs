use gtk::prelude::*;

#[derive(Debug)]
pub enum Msg {
    Update,
}

pub struct Model {
    tasks: relm4::Controller<crate::widgets::tasks::Model>,
}

impl Model {
    fn update_tasks(&mut self) {
        use relm4::ComponentController as _;

        let list = crate::application::tasks();
        let tasks = list.tasks.iter().filter(|x| x.finished).cloned().collect();

        self.tasks
            .sender()
            .emit(crate::widgets::tasks::Msg::Update(tasks));
    }
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Model {
    type Init = ();
    type Input = Msg;
    type Output = crate::widgets::task::MsgOutput;

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        use relm4::Component as _;
        use relm4::ComponentController as _;

        let tasks = crate::widgets::tasks::Model::builder()
            .launch(())
            .forward(sender.output_sender(), std::convert::identity);

        let model = Self { tasks };

        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match msg {
            Msg::Update => self.update_tasks(),
        }
    }

    view! {
        gtk::Box {
            append: model.tasks.widget(),
        }
    }
}
