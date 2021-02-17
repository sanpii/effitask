use gtk::prelude::*;

type ChannelData = (log::Level, String);
type Sender = std::sync::mpsc::Sender<ChannelData>;
type Receiver = std::sync::mpsc::Receiver<ChannelData>;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Add(ChannelData),
    Clear,
    Destroy,
    Hide,
    Read(gtk::ListBoxRow),
    Show,
    Update(gtk::ListBox),
}

pub struct Model {
    relm: relm::Relm<Widget>,
}

pub struct Log {
    tx: std::sync::Mutex<Sender>,
}

impl Log {
    pub fn new(tx: Sender) -> Self {
        Self {
            tx: std::sync::Mutex::new(tx),
        }
    }
}

impl log::Log for Log {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.target() == crate::application::NAME && metadata.level() >= log::Level::Info
    }

    fn log(&self, record: &log::Record<'_>) {
        if let Ok(tx) = self.tx.lock() {
            tx.send((record.level(), format!("{}", record.args())))
                .unwrap_or_default();
        }
    }

    fn flush(&self) {}
}

thread_local!(
    static GLOBAL: std::cell::RefCell<Option<(relm::StreamHandle<Msg>, Receiver)>>
        = std::cell::RefCell::new(None)
);

impl Widget {
    fn receive() -> glib::Continue {
        GLOBAL.with(|global| {
            if let Some((ref stream, ref rx)) = *global.borrow() {
                if let Ok(data) = rx.try_recv() {
                    stream.emit(Msg::Add(data));
                }
            }
        });

        glib::Continue(true)
    }

    fn add_message(&self, level: log::Level, text: &str) {
        let class = format!("{}", level);

        let label = gtk::Label::new(Some(text));
        label.show();

        let context = label.get_style_context();
        context.add_class(&class.to_lowercase());

        self.widgets.list_box.add(&label);
    }

    fn hide(&self) {
        self.widgets.popover.hide();
        self.widgets.toggle.set_active(false);
    }

    fn show(&self) {
        self.widgets.popover.show();
    }

    fn update_count(&self, list_box: &gtk::ListBox)
    {
        use std::str::FromStr;

        let count = list_box.get_children().len();
        if count == 0 {
            self.widgets.toggle.hide();
        } else {
            self.widgets.toggle.show();
        };
        self.widgets.count.set_label(&format!("{}", count));

        let mut max_level = log::Level::Trace;

        for row in list_box.get_children() {
            let label = match row.downcast::<gtk::Bin>().unwrap().get_child() {
                Some(label) => label,
                None => continue,
            };
            let context = label.get_style_context();
            let level = context.list_classes()
                .iter()
                .find_map(|class| log::Level::from_str(&class).ok())
                .unwrap_or(log::Level::Info);

            if level < max_level {
                max_level = level;
            }

            if max_level == log::Level::Error {
                break;
            }
        }

        let context = self.widgets.count.get_style_context();
        context.add_class(&format!("{}", max_level).to_lowercase());
    }
}

#[relm_derive::widget]
impl relm::Widget for Widget {
    fn init_view(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        let log = Log::new(tx);

        log::set_max_level(log::LevelFilter::Info);
        log::set_boxed_logger(Box::new(log)).unwrap_or_default();

        let context = self.widgets.popover.get_style_context();
        context.add_class("log");

        let context = self.widgets.toggle.get_style_context();
        context.add_class("log");

        let context = self.widgets.count.get_style_context();
        context.add_class("count");

        let stream = self.model.relm.stream().clone();
        GLOBAL.with(move |global| *global.borrow_mut() = Some((stream, rx)));
        glib::idle_add(Self::receive);
    }

    fn model(relm: &relm::Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Add((level, text)) => self.add_message(level, &text),
            Clear => self.widgets.list_box.foreach(|row| self.widgets.list_box.remove(row)),
            Destroy => GLOBAL.with(move |global| *global.borrow_mut() = None),
            Hide => self.hide(),
            Read(row) => self.widgets.list_box.remove(&row),
            Show => self.show(),
            Update(sender) => self.update_count(&sender),
        }
    }

    view! {
        #[name="toggle"]
        gtk::ToggleButton {
            gtk::Box {
                spacing: 10,
                gtk::Label {
                    label: "Notifications",
                },
                #[name="count"]
                gtk::Label {
                    label: "0",
                },
                gtk::Image {
                    property_icon_name: Some("go-down-symbolic"),
                },
            },
            toggled(e) => if e.get_active() {
                Msg::Show
            } else {
                Msg::Hide
            },
        }

        #[name="popover"]
        gtk::Popover {
            property_height_request: 500,
            border_width: 5,
            relative_to: Some(&toggle),
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                gtk::ScrolledWindow {
                    property_hscrollbar_policy: gtk::PolicyType::Never,
                    property_vscrollbar_policy: gtk::PolicyType::Automatic,
                    child: {
                        fill: true,
                        expand: true,
                    },
                    #[name="list_box"]
                    gtk::ListBox {
                        row_activated(_, row) => Msg::Read(row.clone()),
                        add(sender, _) => Msg::Update(sender.clone()),
                        remove(sender, _) => Msg::Update(sender.clone()),
                        destroy(_) => Msg::Destroy,
                    },
                },
                gtk::Button {
                    label: "Clear all",
                    image: Some(&gtk::Image::from_icon_name(Some("list-remove-all"), gtk::IconSize::SmallToolbar)),
                    clicked(_) => Msg::Clear,
                },
            },
            hide(_) => Msg::Hide,
        }
    }
}
