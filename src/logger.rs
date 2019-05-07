use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

type ChannelData = (::log::Level, String);
type Sender = ::std::sync::mpsc::Sender<ChannelData>;
type Receiver = ::std::sync::mpsc::Receiver<ChannelData>;

#[derive(Msg)]
pub enum Msg {
    Hide,
}

pub struct Log {
    tx: ::std::sync::Mutex<Sender>,
}

impl Log {
    pub fn new(tx: Sender) -> Self {
        Self {
            tx: ::std::sync::Mutex::new(tx),
        }
    }
}

impl ::log::Log for Log {
    fn enabled(&self, metadata: &::log::Metadata) -> bool {
        metadata.target() == crate::application::NAME && metadata.level() >= ::log::Level::Info
    }

    fn log(&self, record: &::log::Record) {
        if let Ok(tx) = self.tx.lock() {
            tx.send((record.level(), format!("{}", record.args())))
                .unwrap_or_default();
        }
    }

    fn flush(&self) {}
}

thread_local!(
    static GLOBAL: ::std::cell::RefCell<Option<(::gtk::ListBox, Receiver)>>
        = ::std::cell::RefCell::new(None)
);

impl Widget {
    fn init(&self) {
        let (tx, rx) = ::std::sync::mpsc::channel();
        let log = Log::new(tx);

        ::log::set_max_level(::log::LevelFilter::Info);
        ::log::set_boxed_logger(Box::new(log)).unwrap_or_default();

        self.revealer.set_border_width(10);
        self.revealer.show();

        let context = self.revealer.get_style_context();
        context.add_class("log");

        let list_box = ::gtk::ListBox::new();
        connect!(
            self.model,
            list_box,
            connect_button_press_event(_, _),
            return (Msg::Hide, ::gtk::Inhibit(false))
        );
        list_box.show();
        self.revealer.add(&list_box);

        GLOBAL.with(move |global| *global.borrow_mut() = Some((list_box, rx)));

        ::std::thread::spawn(move || loop {
            ::std::thread::sleep(::std::time::Duration::from_millis(100));
            ::glib::idle_add(Self::receive);
        });
    }

    fn receive() -> ::glib::Continue {
        GLOBAL.with(|global| {
            if let Some((ref list_box, ref rx)) = *global.borrow() {
                if let Ok((level, text)) = rx.try_recv() {
                    Self::add_message(list_box, level, &text);
                }
            }
        });

        ::glib::Continue(false)
    }

    fn add_message(list_box: &::gtk::ListBox, level: ::log::Level, text: &str) {
        let label = ::gtk::Label::new(Some(text));
        label.show();
        list_box.add(&label);

        let context = label.get_style_context();

        for class in context.list_classes() {
            context.remove_class(&class);
        }

        let class = format!("{}", level);
        context.add_class(&class.to_lowercase());

        if let Some(parent) = list_box.get_parent() {
            let revealer = parent.downcast::<::gtk::Revealer>().unwrap();

            revealer.set_reveal_child(true);
        }
    }

    fn hide(&self) {
        self.revealer.set_reveal_child(false);

        if let Some(parent) = self.revealer.get_child() {
            let list_box = parent.downcast::<::gtk::ListBox>().unwrap();

            for child in list_box.get_children() {
                child.destroy();
            }
        }
    }
}

#[widget]
impl ::relm::Widget for Widget {
    fn init_view(&mut self) {
        self.init();
    }

    fn model(relm: &::relm::Relm<Self>, _: ()) -> ::relm::Relm<Widget> {
        relm.clone()
    }

    fn update(&mut self, event: Msg) {
        use self::Msg::*;

        match event {
            Hide => self.hide(),
        }
    }

    view!
    {
        #[name="revealer"]
        gtk::Revealer {
        }
    }
}
