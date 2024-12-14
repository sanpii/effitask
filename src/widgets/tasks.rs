use gtk::prelude::*;

#[derive(Debug)]
pub enum Msg {
    Update(Vec<crate::tasks::Task>),
}

pub struct Model {
    children: Vec<relm4::Controller<super::task::Model>>,
    list_box: gtk::ListBox,
}

impl Model {
    fn update_tasks(&mut self, sender: relm4::ComponentSender<Self>, tasks: &[crate::tasks::Task]) {
        use relm4::Component as _;
        use relm4::ComponentController as _;

        self.clear();

        if tasks.is_empty() {
            self.list_box.set_visible(false);
            return;
        }

        let mut sorted_tasks = tasks.to_owned();
        sorted_tasks.sort();
        sorted_tasks.reverse();

        for task in &sorted_tasks {
            let child = super::task::Model::builder()
                .launch(task.clone())
                .forward(sender.output_sender(), std::convert::identity);

            self.list_box.append(child.widget());

            self.children.push(child);
        }
    }

    fn clear(&mut self) {
        self.list_box.remove_all();
        self.children = Vec::new();
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
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let list_box = gtk::ListBox::new();
        list_box.set_hexpand(true);
        list_box.set_vexpand(true);

        let model = Self {
            children: Vec::new(),
            list_box,
        };

        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: relm4::ComponentSender<Self>) {
        use Msg::*;

        match msg {
            Update(tasks) => self.update_tasks(sender, &tasks),
        }
    }

    view! {
        gtk::ScrolledWindow {
            gtk::Box {
                append: &model.list_box,

                gtk::Label {
                    #[watch]
                    set_visible: model.children.is_empty(),
                    set_hexpand: true,
                    set_vexpand: true,
                    set_text: "Nothing to do :)",
                },
            },
        },
    }
}
