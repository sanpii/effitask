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

impl Log
{
    pub fn new(tx: Sender) -> Self
    {
        Self {
            tx: ::std::sync::Mutex::new(tx),
        }
    }
}

impl ::log::Log for Log
{
    fn enabled(&self, metadata: &::log::Metadata) -> bool
    {
        metadata.target() == ::application::NAME
            && metadata.level() >= ::log::Level::Info
    }

    fn log(&self, record: &::log::Record)
    {
        if let Ok(tx) = self.tx.lock() {
            tx.send((record.level(), format!("{}", record.args())))
                .unwrap_or_default();
        }
    }

    fn flush(&self)
    {
    }
}

thread_local!(
    static GLOBAL: ::std::cell::RefCell<Option<(::gtk::Revealer, Receiver)>> = ::std::cell::RefCell::new(None)
);

impl Widget
{
    fn init(&self)
    {
        let (tx, rx) = ::std::sync::mpsc::channel();
        let log = Log::new(tx);

        ::log::set_max_level(::log::LevelFilter::Info);
        ::log::set_boxed_logger(Box::new(log))
            .unwrap_or_default();

        let revealer = ::gtk::Revealer::new();
        revealer.set_border_width(10);
        revealer.show();
        self.event_box.add(&revealer);

        let context = revealer.get_style_context()
            .unwrap();
        context.add_class("log");

        let label = ::gtk::Label::new(None);
        label.show();
        revealer.add(&label);

        GLOBAL.with(move |global| {
            *global.borrow_mut() = Some((revealer, rx))
        });

        ::std::thread::spawn(move || {
            loop {
                ::std::thread::sleep(::std::time::Duration::from_millis(100));
                ::glib::idle_add(Self::receive);
            }
        });
    }

    fn receive() -> ::glib::Continue
    {
        GLOBAL.with(|global| {
            if let Some((ref revealer, ref rx)) = *global.borrow() {
                if let Ok((level, text)) = rx.try_recv() {
                    Self::add_message(revealer, level, &text);
                }
            }

        });

        ::glib::Continue(false)
    }

    fn add_message(revealer: &::gtk::Revealer, level: ::log::Level, text: &str)
    {
        use gtk::StyleContextExt;

        if let Some(child) = revealer.get_child() {
            let label = child.downcast::<::gtk::Label>()
                .unwrap();
            label.set_text(text);

            let context = label.get_style_context()
                .unwrap();

            for class in context.list_classes() {
                context.remove_class(&class);
            }

            let class = format!("{}", level);
            context.add_class(&class.to_lowercase());

            revealer.set_reveal_child(true);
        }
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.init();
    }

    fn model(_: ()) -> ()
    {
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Hide => if let Some(child) = self.event_box.get_child() {
                let revealer = child.downcast::<::gtk::Revealer>()
                    .unwrap();
                revealer.set_reveal_child(false);
            },
        }
    }

    view!
    {
        #[name="event_box"]
        gtk::EventBox {
            button_press_event(_, _) => (Msg::Hide, ::gtk::Inhibit(false)),
        }
    }
}
