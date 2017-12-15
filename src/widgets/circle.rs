use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Draw,
}

impl Circle
{
    fn draw(&self)
    {
        let context = self.create_context();
        let task = &self.model;
        let center = self.center();

        if let Some(due_date) = task.due_date {
            let today = ::chrono::Local::now()
                .date()
                .naive_local();

            if due_date < today {
                context.set_source_rgb(1., 0.4, 0.5);
            }
            else {
                context.set_source_rgb(1., 0.8, 0.2);
            }
        }
        else {
            context.set_source_rgb(0.8, 0.8, 0.8);
        }

        context.set_line_width(8.);
        context.arc(center, center, center - 5., 0., 2. * ::std::f64::consts::PI);
        context.close_path();

        if task.finished {
            context.fill_preserve();
        }

        context.stroke();

    }

    fn center(&self) -> f64
    {
        f64::min(
            f64::from(self.drawing_area.get_property_width_request()) / 2.,
            f64::from(self.drawing_area.get_property_height_request()) / 2.,
        )
    }

    fn create_context(&self) -> ::cairo::Context
    {
        let window = self.drawing_area.get_window()
            .unwrap();

        unsafe {
            use glib::translate::ToGlibPtr;
            use glib::translate::FromGlibPtrNone;

            let ptr = ::gdk_sys::gdk_cairo_create(window.to_glib_none().0);

            ::cairo::Context::from_glib_none(ptr)
        }
    }
}

#[widget]
impl ::relm::Widget for Circle
{
    fn model(task: ::tasks::Task) -> ::tasks::Task
    {
        task
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Draw => self.draw(),
        }
    }

    view!
    {
        #[name="drawing_area"]
        gtk::DrawingArea {
            property_height_request: 60,
            property_width_request: 60,
            draw(_, _) => (Msg::Draw, ::gtk::Inhibit(false)),
        }
    }
}
