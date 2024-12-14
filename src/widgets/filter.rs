#![allow(deprecated)]

use gtk::prelude::*;
use relm4::ComponentController as _;

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

#[derive(Debug)]
pub enum MsgInput {
    SelectionChange,
    UpdateFilters(Vec<(String, (u32, u32))>),
    UpdateTasks(Vec<crate::tasks::Task>),
}

#[derive(Debug)]
pub enum MsgOutput {
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    Filters(Vec<String>),
}

pub struct Model {
    store: gtk::TreeStore,
    filters: std::collections::BTreeMap<gtk::TreePath, String>,
    tasks: relm4::Controller<super::tasks::Model>,
    tree_view: gtk::TreeView,
}

impl Model {
    fn update_filters(&mut self, filters: Vec<(String, (u32, u32))>) {
        let selection = self.tree_view.selection();
        let (paths, _) = selection.selected_rows();

        self.filters.clear();
        self.store.clear();
        let mut root = std::collections::HashMap::new();

        for filter in filters {
            self.append(&mut root, filter);
        }

        self.tree_view.expand_all();

        for path in paths {
            gtk::prelude::TreeViewExt::set_cursor(&self.tree_view, &path, None, false);
        }
    }

    fn append(
        &mut self,
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

        let row = self.store.append(root.get(&parent));

        self.store
            .set_value(&row, Column::Title.into(), &title.to_value());
        self.store
            .set_value(&row, Column::Raw.into(), &filter.to_value());
        self.store
            .set_value(&row, Column::Progress.into(), &progress.to_value());

        let tooltip = format!("{done}/{total}");
        self.store
            .set_value(&row, Column::Tooltip.into(), &tooltip.to_value());

        root.insert(filter.clone(), row);

        let path = self.store.path(&row);
        self.filters.insert(path, filter);
    }

    fn update_tasks(&self, tasks: Vec<crate::tasks::Task>) {
        self.tasks.emit(super::tasks::Msg::Update(tasks));
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
            let end = model.path(&end_iter);

            treeview.selection().select_range(start, &end);
        }
    }
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Model {
    type Init = ();
    type Input = MsgInput;
    type Output = MsgOutput;

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        use relm4::Component as _;

        let tasks = crate::widgets::tasks::Model::builder().launch(()).forward(
            sender.output_sender(),
            |output| match output {
                super::task::MsgOutput::Complete(task) => MsgOutput::Complete(task),
                super::task::MsgOutput::Edit(task) => MsgOutput::Edit(task),
            },
        );

        let columns = vec![
            gtk::glib::types::Type::STRING,
            gtk::glib::types::Type::STRING,
            gtk::glib::types::Type::U32,
            gtk::glib::types::Type::STRING,
        ];

        let model = Self {
            tasks,
            filters: std::collections::BTreeMap::new(),
            store: gtk::TreeStore::new(&columns),
            tree_view: gtk::TreeView::new(),
        };

        let filters = &model.tree_view;
        let widgets = view_output!();

        filters.set_model(Some(&model.store));

        let selection = filters.selection();
        selection.set_mode(gtk::SelectionMode::Multiple);
        selection.connect_changed(move |_| {
            sender.input(MsgInput::SelectionChange);
        });

        let column = gtk::TreeViewColumn::new();
        filters.append_column(&column);

        let cell = gtk::CellRendererProgress::new();
        cell.set_text_xalign(0.);
        gtk::prelude::CellLayoutExt::pack_start(&column, &cell, true);
        column.add_attribute(&cell, "text", Column::Title.into());
        column.add_attribute(&cell, "value", Column::Progress.into());

        filters.set_tooltip_column(Column::Tooltip.into());

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: relm4::ComponentSender<Self>) {
        use MsgInput::*;

        match msg {
            SelectionChange => {
                let mut filters = Vec::new();

                let (paths, _) = self.tree_view.selection().selected_rows();

                for path in paths {
                    match self.filters.get(&path) {
                        Some(value) => filters.push(value.clone()),
                        None => continue,
                    };
                }

                sender.output(MsgOutput::Filters(filters)).ok();
            }
            UpdateFilters(filters) => self.update_filters(filters),
            UpdateTasks(tasks) => self.update_tasks(tasks),
        }
    }

    view! {
        gtk::Paned {
            set_orientation: gtk::Orientation::Horizontal,
            set_position: 200,
            set_wide_handle: true,

            #[wrap(Some)]
            set_start_child = &gtk::ScrolledWindow {
                set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),

                #[local_ref]
                filters -> gtk::TreeView {
                    set_enable_tree_lines: true,
                    set_headers_visible: false,

                    connect_row_activated => |treeview, path, _| Self::select_range(treeview, path),
                },
            },
            #[wrap(Some)]
            set_end_child = &gtk::ScrolledWindow {
                set_child: Some(model.tasks.widget()),
            },
        }
    }
}
