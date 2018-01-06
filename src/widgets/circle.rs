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

        if task.finished || task.due_date.is_none() {
            context.set_source_rgb(0.8, 0.8, 0.8);
        }
        else {
            let due_date = task.due_date.unwrap();
            let today = ::date::today();

            if due_date < today {
                context.set_source_rgb(1., 0.4, 0.5);
            }
            else {
                context.set_source_rgb(1., 0.8, 0.2);
            }
        }

        context.set_line_width(8.);
        context.arc(center, center, center - 5., 0., 2. * ::std::f64::consts::PI);
        context.close_path();

        if task.finished {
            context.fill_preserve();
        }

        context.stroke();

        if !task.finished && task.flagged {
            let angle = if task.due_date.is_some() {
                ::std::f64::consts::PI
            }
            else {
                0.
            };

            context.set_source_rgb(1., 0.5, 0.3);
            context.arc(center, center, center - 5., angle, 2. * ::std::f64::consts::PI);
            context.stroke();
        }

        if task.recurrence.is_some() {
            context.set_line_width(2.);

            for dx in &[-12., 0., 12.] {
                context.arc(center + dx, center, 4., 0., 2. * ::std::f64::consts::PI);
                context.close_path();
                context.stroke();
            }
        }
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
