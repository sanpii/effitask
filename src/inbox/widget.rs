use gtk::{self, WidgetExt};

use relm_attributes::widget;

#[widget]
impl ::relm::Widget for Widget
{
    fn model() -> ()
    {
        ()
    }

    fn update(&mut self, _: ())
    {
    }

    view!
    {
        gtk::ScrolledWindow {
            gtk::ListBox {
            }
        }
    }
}
