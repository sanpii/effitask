use gtk::prelude::*;
use relm4::ComponentController as _;

#[derive(Debug)]
pub enum MsgInput {
    Update,
    UpdateFilter(String),
}

pub struct Model {
    query: String,
    tasks: relm4::Controller<crate::widgets::tasks::Model>,
}

impl Model {
    fn update_tasks(&mut self) {
        self.update();
    }

    fn update_filter(&mut self, filter: &str) {
        self.query = filter.to_string();
        self.update();
    }

    fn update(&self) {
        let filter = self.query.to_lowercase();
        let list = crate::application::tasks();

        let tasks = list
            .tasks
            .iter()
            .filter(|x| x.subject.to_lowercase().contains(filter.as_str()))
            .cloned()
            .collect();

        self.tasks.emit(crate::widgets::tasks::Msg::Update(tasks));
    }
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Model {
    type Init = String;
    type Input = MsgInput;
    type Output = crate::widgets::task::MsgOutput;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        use relm4::Component as _;

        let tasks = crate::widgets::tasks::Model::builder()
            .launch(())
            .forward(sender.output_sender(), std::convert::identity);

        let model = Self { query: init, tasks };

        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: relm4::ComponentSender<Self>) {
        use MsgInput::*;

        match msg {
            Update => self.update_tasks(),
            UpdateFilter(filter) => self.update_filter(&filter),
        }
    }

    view! {
        gtk::Box {
            append: model.tasks.widget(),
        }
    }
}
