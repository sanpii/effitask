use gtk::prelude::*;

type ChannelData = (log::Level, String);
type Sender = std::sync::mpsc::Sender<ChannelData>;
type Receiver = std::sync::mpsc::Receiver<ChannelData>;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Clear,
    Hide,
    Read(gtk::ListBoxRow),
    Show,
    Update(gtk::ListBox),
}

pub struct Model {
    relm: relm::Relm<Widget>,
    popover: gtk::Popover,
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
            tx.send((record.level(), format!("{}", record.args()))).ok();
        }
    }

    fn flush(&self) {}
}

thread_local!(
    static GLOBAL: std::cell::RefCell<Option<(gtk::ListBox, Receiver)>>
        = const { std::cell::RefCell::new(None) }
);

impl Widget {
    fn init(&self) {
        let (tx, rx) = std::sync::mpsc::channel();
        let log = Log::new(tx);

        log::set_max_level(log::LevelFilter::Info);
        log::set_boxed_logger(Box::new(log)).unwrap_or_default();

        let popover = &self.model.popover;
        popover.set_height_request(500);
        popover.set_relative_to(Some(&self.widgets.toggle));
        popover.set_border_width(5);
        relm::connect!(self.model.relm, popover, connect_hide(_), Msg::Hide);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        popover.add(&vbox);

        let context = popover.style_context();
        context.add_class("log");

        let scrolled_window =
            gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        vbox.pack_start(&scrolled_window, true, true, 0);

        let list_box = gtk::ListBox::new();
        relm::connect!(
            self.model.relm,
            list_box,
            connect_row_activated(_, row),
            Msg::Read(row.clone())
        );
        relm::connect!(
            self.model.relm,
            list_box,
            connect_add(sender, _),
            Msg::Update(sender.clone())
        );
        relm::connect!(
            self.model.relm,
            list_box,
            connect_remove(sender, _),
            Msg::Update(sender.clone())
        );
        scrolled_window.add(&list_box);

        let clear = gtk::Button::with_label("Clear all");
        clear.set_image(Some(&gtk::Image::from_icon_name(
            Some("list-remove-all"),
            gtk::IconSize::SmallToolbar,
        )));
        relm::connect!(self.model.relm, clear, connect_clicked(_), Msg::Clear);
        vbox.pack_start(&clear, false, false, 0);

        vbox.show_all();

        GLOBAL.with(move |global| *global.borrow_mut() = Some((list_box, rx)));
        gtk::glib::idle_add(Self::receive);
    }

    fn receive() -> gtk::glib::Continue {
        GLOBAL.with(|global| {
            if let Some((ref list_box, ref rx)) = *global.borrow() {
                if let Ok((level, text)) = rx.try_recv() {
                    Self::add_message(list_box, level, &text);
                }
            }
        });

        gtk::glib::Continue(true)
    }

    fn add_message(list_box: &gtk::ListBox, level: log::Level, text: &str) {
        let class = level.to_string();

        let label = gtk::Label::new(Some(text));
        label.show();

        let context = label.style_context();
        context.add_class(&class.to_lowercase());

        list_box.add(&label);
    }

    fn clear(&self) {
        GLOBAL.with(|global| {
            if let Some((ref list_box, _)) = *global.borrow() {
                list_box.foreach(|row| list_box.remove(row));
            }
        });
    }

    fn hide(&self) {
        self.model.popover.hide();
        self.widgets.toggle.set_active(false);
    }

    fn read(&self, row: &gtk::ListBoxRow) {
        GLOBAL.with(|global| {
            if let Some((ref list_box, _)) = *global.borrow() {
                list_box.remove(row);
            }
        });
    }

    fn show(&self) {
        self.model.popover.show();
    }

    fn update_count(&self, list_box: &gtk::ListBox) {
        use std::str::FromStr;

        let count = list_box.children().len();
        if count == 0 {
            self.widgets.toggle.hide();
        } else {
            self.widgets.toggle.show();
        };
        self.widgets.count.set_label(&count.to_string());

        let mut max_level = log::Level::Trace;

        for row in list_box.children() {
            let Some(label) = row.downcast::<gtk::Bin>().unwrap().child() else {
                continue;
            };
            let context = label.style_context();
            let level = context
                .list_classes()
                .iter()
                .find_map(|class| log::Level::from_str(class).ok())
                .unwrap_or(log::Level::Info);

            if level < max_level {
                max_level = level;
            }

            if max_level == log::Level::Error {
                break;
            }
        }

        let context = self.widgets.count.style_context();
        context.add_class(&max_level.to_string().to_lowercase());
    }
}

#[relm_derive::widget]
impl relm::Widget for Widget {
    fn init_view(&mut self) {
        self.init();
    }

    fn model(relm: &relm::Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            popover: gtk::Popover::new(None::<&gtk::Button>),
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Clear => self.clear(),
            Hide => self.hide(),
            Read(row) => self.read(&row),
            Show => self.show(),
            Update(sender) => self.update_count(&sender),
        }
    }

    view! {
        #[name="toggle"]
        #[style_name="log"]
        gtk::ToggleButton {
            gtk::Box {
                spacing: 10,
                gtk::Label {
                    label: "Notifications",
                },
                #[name="count"]
                #[style_name="count"]
                gtk::Label {
                    label: "0",
                },
                gtk::Image {
                    icon_name: Some("go-down-symbolic"),
                },
            },
            toggled(e) => if e.is_active() {
                Msg::Show
            } else {
                Msg::Hide
            },
        }
    }
}
