use crate::widgets::tasks::Msg::{Complete, Edit};
use crate::widgets::Tasks;
use gtk::prelude::*;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    Filters(Vec<String>),
    UpdateFilters(Vec<(String, (u32, u32))>),
    UpdateTasks(Vec<crate::tasks::Task>),
}

#[repr(u32)]
enum Column {
    Title = 0,
    Raw = 1,
    Progress = 2,
    Tooltip = 3,
}

impl From<Column> for u32 {
    fn from(column: Column) -> u32 {
        unsafe { std::mem::transmute(column) }
    }
}

impl From<Column> for i32 {
    fn from(column: Column) -> i32 {
        unsafe { std::mem::transmute(column) }
    }
}

impl Filter {
    fn update_filters(&mut self, filters: Vec<(String, (u32, u32))>) {
        let selection = self.widgets.filters.selection();
        let (paths, _) = selection.selected_rows();

        self.model.clear();
        let mut root = std::collections::HashMap::new();

        for filter in filters {
            self.append(&mut root, filter);
        }

        self.widgets.filters.expand_all();

        for path in paths {
            self.widgets
                .filters
                .set_cursor(&path, None as Option<&gtk::TreeViewColumn>, false);
        }
    }

    fn append(
        &self,
        root: &mut std::collections::HashMap<String, gtk::TreeIter>,
        filter: (String, (u32, u32)),
    ) {
        let separator = '\\';
        let (filter, (done, total)) = filter;
        let progress = (done as f32 / total as f32) * 100.;
        let f = filter.clone();

        let mut levels: Vec<_> = f.split(separator).collect();
        let title = levels.pop().unwrap();

        let parent = levels.join(&separator.to_string());

        if !parent.is_empty() && root.get(&parent).is_none() {
            self.append(root, (parent.clone(), (0, 0)));
        }

        let row = self.model.append(root.get(&parent));

        self.model
            .set_value(&row, Column::Title.into(), &title.to_value());
        self.model
            .set_value(&row, Column::Raw.into(), &filter.to_value());
        self.model
            .set_value(&row, Column::Progress.into(), &progress.to_value());

        let tooltip = format!("{}/{}", done, total);
        self.model
            .set_value(&row, Column::Tooltip.into(), &tooltip.to_value());

        root.insert(filter, row);
    }

    fn update_tasks(&self, tasks: Vec<crate::tasks::Task>) {
        self.components
            .tasks
            .emit(crate::widgets::tasks::Msg::Update(tasks));
    }

    fn select_range(treeview: &gtk::TreeView, path: &gtk::TreePath) {
        let model = treeview.model().unwrap();

        let start = path;
        let start_iter = model.iter(path).unwrap();

        let n_child = model.iter_n_children(Some(&start_iter));

        if n_child > 0 {
            let end_iter = model
                .iter_nth_child(Some(&start_iter), n_child - 1)
                .unwrap();
            let end = model.path(&end_iter).unwrap();

            treeview.selection().select_range(start, &end);
        }
    }
}

#[relm_derive::widget]
impl relm::Widget for Filter {
    fn init_view(&mut self) {
        self.widgets.filters.set_size_request(200, -1);
        self.widgets
            .scroll
            .set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        self.widgets.filters.set_model(Some(&self.model));
        self.widgets
            .filters
            .selection()
            .set_mode(gtk::SelectionMode::Multiple);

        let column = gtk::TreeViewColumn::new();
        self.widgets.filters.append_column(&column);

        let cell = gtk::CellRendererProgress::new();
        cell.set_text_xalign(0.);
        gtk::prelude::CellLayoutExt::pack_start(&column, &cell, true);
        gtk::prelude::TreeViewColumnExt::add_attribute(
            &column,
            &cell,
            "text",
            Column::Title.into(),
        );
        gtk::prelude::TreeViewColumnExt::add_attribute(
            &column,
            &cell,
            "value",
            Column::Progress.into(),
        );

        self.widgets
            .filters
            .set_tooltip_column(Column::Tooltip.into());
    }

    fn model(_: ()) -> gtk::TreeStore {
        let columns = vec![
            gtk::glib::types::Type::STRING,
            gtk::glib::types::Type::STRING,
            gtk::glib::types::Type::U32,
            gtk::glib::types::Type::STRING,
        ];

        gtk::TreeStore::new(&columns)
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Filters(_) => (),
            UpdateFilters(filters) => self.update_filters(filters),
            UpdateTasks(tasks) => self.update_tasks(tasks),
        }
    }

    view! {
        gtk::Paned {
            orientation: gtk::Orientation::Horizontal,
            wide_handle: true,
            #[name="scroll"]
            gtk::ScrolledWindow {
                #[name="filters"]
                gtk::TreeView {
                    headers_visible: false,
                    enable_tree_lines: true,
                    row_activated(treeview, path, _) => Self::select_range(treeview, path),
                    selection.changed(ref mut selection) => {
                        let mut filters = Vec::new();
                        let (paths, list_model) = selection.selected_rows();

                        for path in paths {
                            let iter = match list_model.iter(&path) {
                                Some(iter) => iter,
                                None => continue,
                            };

                            match list_model.value(&iter, Column::Raw.into()).get() {
                                Ok(Some(value)) => filters.push(value),
                                Ok(None) | Err(_) => continue,
                            };
                        }

                        Msg::Filters(filters)
                    },
                }
            },
            gtk::ScrolledWindow {
                #[name="tasks"]
                Tasks {
                    Complete(ref task) => Msg::Complete(task.clone()),
                    Edit(ref task) => Msg::Edit(task.clone()),
                },
            }
        }
    }
}
