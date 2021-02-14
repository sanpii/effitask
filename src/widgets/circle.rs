use gtk::prelude::*;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Draw,
}

pub struct Model {
    draw_handler: relm::DrawHandler<gtk::DrawingArea>,
    task: crate::tasks::Task,
}

impl Circle {
    fn draw(&mut self) {
        let context = self.model.draw_handler.get_context();
        let task = &self.model.task;
        let center = self.center();

        if task.finished || task.due_date.is_none() {
            context.set_source_rgb(0.8, 0.8, 0.8);
        } else {
            let due_date = task.due_date.unwrap();
            let today = crate::date::today();

            if due_date < today {
                context.set_source_rgb(1., 0.4, 0.5);
            } else {
                context.set_source_rgb(1., 0.8, 0.2);
            }
        }

        context.set_line_width(8.);
        context.arc(center, center, center - 5., 0., 2. * std::f64::consts::PI);
        context.close_path();

        if task.finished {
            let width = self.widgets.drawing_area.get_property_width_request();
            let height = self.widgets.drawing_area.get_property_height_request();

            context.save();
            context.fill();
            context.translate(f64::from(width) as f64 / -4., f64::from(height) as f64 / 2.);
            context.rotate(std::f64::consts::PI / -4.);
            context.set_source_rgb(0., 0., 0.);
            context.rectangle(20., 30., 40., 10.);
            context.rectangle(20., 20., 10., 10.);
            context.fill();
            context.restore();
        }

        context.stroke();

        if !task.finished && task.flagged {
            let angle = if task.due_date.is_some() {
                std::f64::consts::PI
            } else {
                0.
            };

            context.set_source_rgb(1., 0.5, 0.3);
            context.arc(
                center,
                center,
                center - 5.,
                angle,
                2. * std::f64::consts::PI,
            );
            context.stroke();
        }

        if !task.finished && task.recurrence.is_some() {
            context.set_line_width(2.);

            for dx in &[-12., 0., 12.] {
                context.arc(center + dx, center, 4., 0., 2. * std::f64::consts::PI);
                context.close_path();
                context.stroke();
            }
        }
    }

    fn center(&self) -> f64 {
        f64::min(
            f64::from(self.widgets.drawing_area.get_property_width_request()) / 2.,
            f64::from(self.widgets.drawing_area.get_property_height_request()) / 2.,
        )
    }
}

#[relm_derive::widget]
impl relm::Widget for Circle {
    fn init_view(&mut self) {
        self.model.draw_handler.init(&self.widgets.drawing_area);
    }

    fn model(task: crate::tasks::Task) -> Model {
        Model {
            draw_handler: relm::DrawHandler::new().expect("draw handler"),
            task,
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Draw => self.draw(),
        }
    }

    view! {
        #[name="drawing_area"]
        gtk::DrawingArea {
            property_height_request: 60,
            property_width_request: 60,
            draw(_, _) => (Msg::Draw, gtk::Inhibit(false)),
        }
    }
}
