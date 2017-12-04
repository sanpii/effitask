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

impl Tasks
{
    pub fn populate(&mut self, tasks: Vec<::todo_txt::Task>)
    {
        self.model.clear();

        for task in tasks.iter() {
            let row = self.model.append();
            self.model.set_value(&row, 0, &gtk::Value::from(&task.subject));
        }
    }
}

#[widget]
impl ::relm::Widget for Tasks
{
    fn init_view(&mut self)
    {
        self.tree_view.set_model(Some(&self.model));

        let cell = gtk::CellRendererText::new();
        let view_column = gtk::TreeViewColumn::new();
        view_column.pack_start(&cell, true);
        view_column.add_attribute(&cell, "text", 0);
        self.tree_view.append_column(&view_column);
    }

    fn model(_: ()) -> gtk::ListStore
    {
        let columns = vec![gtk::Type::String];

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
