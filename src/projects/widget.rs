use gtk::{
    self,
    CellLayoutExt,
    ListStoreExt,
    OrientableExt,
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
    SelectionChanged(gtk::TreeSelection),
}

impl Widget
{
    fn populate(&mut self, filter: Option<String>)
    {
        self.populate_projects();
        self.populate_tasks(filter);
    }

    fn populate_projects(&mut self)
    {
        self.model.projects_store.clear();

        for project in self.model.tasks.projects() {
            let row = self.model.projects_store.append(None);
            self.model.projects_store.set_value(&row, 0, &gtk::Value::from(&project));
        }
    }

    fn populate_tasks(&mut self, filter: Option<String>)
    {
        self.model.tasks_store.clear();

        let mut tasks = Vec::new();

        for task in self.model.tasks.todo.iter() {
            if !task.projects.is_empty() && (filter.is_none() || task.projects.contains(&filter.clone().unwrap())) {
                tasks.push(task.clone());
            }
        }

        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Widget
{
    fn init_view(&mut self)
    {
        self.projects_view.set_model(Some(&self.model.projects_store));

        let cell = gtk::CellRendererText::new();
        let view_column = gtk::TreeViewColumn::new();
        view_column.pack_start(&cell, true);
        view_column.add_attribute(&cell, "text", 0);
        self.projects_view.append_column(&view_column);

        self.populate(None);
    }

    fn model(tasks: ::tasks::List) -> ::projects::Model
    {
        let columns = vec![gtk::Type::String];

        ::projects::Model {
            tasks: tasks,
            tasks_store: gtk::ListStore::new(&columns),
            projects_store: gtk::TreeStore::new(&columns),
        }
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            SelectionChanged(selection) => {
                if let Some((list_model, iter)) = selection.get_selected() {
                    let project = list_model.get_value(&iter, 0)
                        .get();

                    self.populate_tasks(project);
                }
            }
        }
    }

    view!
    {
        gtk::Paned {
            orientation: gtk::Orientation::Horizontal,
            gtk::ScrolledWindow {
                #[name="projects_view"]
                gtk::TreeView {
                    headers_visible: false,
                    selection.changed(selection) => Msg::SelectionChanged(selection.clone()),
                }
            },
            #[name="tasks"]
            ::widgets::Tasks {
            }
        }
    }
}
