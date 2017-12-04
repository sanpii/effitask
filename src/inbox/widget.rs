use gtk::{
    self,
    CellLayoutExt,
    ListStoreExt,
    ListStoreExtManual,
    TreeViewExt,
    WidgetExt,
};
use relm_attributes::widget;

impl Widget
{
    fn populate_tasks(&mut self)
    {
        for task in self.model.tasks.todo.iter() {
            let row = self.model.list_store.append();
            self.model.list_store.set_value(&row, 0, &gtk::Value::from(&task.subject));
        }
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.tree_view.set_model(Some(&self.model.list_store));

        let cell = gtk::CellRendererText::new();
        let view_column = gtk::TreeViewColumn::new();
        view_column.pack_start(&cell, true);
        view_column.add_attribute(&cell, "text", 0);
        self.tree_view.append_column(&view_column);

        self.populate_tasks();
    }

    fn model(tasks: ::tasks::List) -> ::inbox::Model
    {
        let columns = vec![gtk::Type::String];

        ::inbox::Model {
            tasks: tasks,
            list_store: gtk::ListStore::new(&columns),
        }
    }

    fn update(&mut self, _: ())
    {
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
