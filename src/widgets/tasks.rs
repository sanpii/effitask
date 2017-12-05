use gtk::{
    self,
    CellLayoutExt,
    ListStoreExt,
    ListStoreExtManual,
    TreeViewExt,
    WidgetExt,
};
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Update(Vec<::todo_txt::Task>),
}

#[repr(u32)]
enum Column {
    Finished = 0,
    Subject = 1,
}

impl ::std::convert::Into<u32> for Column
{
    fn into(self) -> u32
    {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

impl ::std::convert::Into<i32> for Column
{
    fn into(self) -> i32
    {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

impl Tasks
{
    pub fn populate(&mut self, tasks: Vec<::todo_txt::Task>)
    {
        use ::gtk::ToValue;

        self.model.clear();

        for task in tasks.iter() {
            let row = self.model.append();
            self.model.set_value(&row, Column::Finished.into(), &task.finished.to_value());
            self.model.set_value(&row, Column::Subject.into(), &task.subject.to_value());
        }
    }
}

#[widget]
impl ::relm::Widget for Tasks
{
    fn init_view(&mut self)
    {
        self.tree_view.set_model(Some(&self.model));

        let column = gtk::TreeViewColumn::new();
        self.tree_view.append_column(&column);

        let cell = gtk::CellRendererToggle::new();
        column.pack_start(&cell, false);
        column.add_attribute(&cell, "active", Column::Finished.into());

        let cell = gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", Column::Subject.into());
    }

    fn model(_: ()) -> gtk::ListStore
    {
        let columns = vec![gtk::Type::Bool, gtk::Type::String];

        gtk::ListStore::new(&columns)
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Update(tasks) => self.populate(tasks),
        }
    }

    view!
    {
        gtk::ScrolledWindow {
            #[name="tree_view"]
            gtk::TreeView {
                headers_visible: false,
            }
        }
    }
}
