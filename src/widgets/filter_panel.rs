use gtk::{
    self,
    CellLayoutExt,
    OrientableExt,
    ScrolledWindowExt,
    TreeSelectionExt,
    TreeModelExt,
    TreeStoreExt,
    TreeStoreExtManual,
    TreeViewExt,
    WidgetExt,
};
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Filter(Option<String>),
    UpdateFilters(Vec<String>),
    UpdateTasks(Vec<::todo_txt::Task>),
}

impl FilterPanel
{
    fn populate_filters(&mut self, filters: Vec<String>)
    {
        self.model.clear();

        for filter in filters {
            let row = self.model.append(None);
            self.model.set_value(&row, 0, &::gtk::Value::from(&filter));
        }
    }
}

#[widget]
impl ::relm::Widget for FilterPanel
{
    fn init_view(&mut self)
    {
        self.filters.set_size_request(200, -1);
        self.scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        self.filters.set_model(Some(&self.model));

        let column = gtk::TreeViewColumn::new();
        self.filters.append_column(&column);

        let cell = gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
    }

    fn model(_: ()) -> gtk::TreeStore
    {
        let columns = vec![gtk::Type::String];

        gtk::TreeStore::new(&columns)
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            UpdateFilters(filters) => self.populate_filters(filters),
            UpdateTasks(tasks) => self.tasks.emit(::widgets::tasks::Msg::Update(tasks)),
            Filter(_) => (),
        }
    }

    view!
    {
        gtk::Paned {
            orientation: gtk::Orientation::Horizontal,
            #[name="scroll"]
            gtk::ScrolledWindow {
                #[name="filters"]
                gtk::TreeView {
                    headers_visible: false,
                    selection.changed(selection) => {
                        if let Some((list_model, iter)) = selection.get_selected() {
                            let filter = list_model.get_value(&iter, 0)
                                .get();

                            Msg::Filter(filter)
                        }
                        else {
                            Msg::Filter(None)
                        }
                    },
                }
            },
            #[name="tasks"]
            ::widgets::Tasks {
            }
        }
    }
}
