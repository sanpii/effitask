use gtk;
use gtk::prelude::*;
use relm_attributes::widget;
use widgets::tasks::Msg::{Complete, Edit};

#[derive(Msg)]
pub enum Msg {
    Complete(::tasks::Task),
    Edit(::tasks::Task),
    Filters(Vec<String>),
    UpdateFilters(Vec<(String, (u32, u32))>),
    UpdateTasks(Vec<::tasks::Task>),
}

#[repr(u32)]
enum Column {
    Title = 0,
    Raw = 1,
    Progress = 2,
    Tooltip = 3,
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

impl Filter
{
    fn update_filters(&mut self, filters: Vec<(String, (u32, u32))>)
    {
        let selection = self.filters.get_selection();
        let (paths, _) = selection.get_selected_rows();

        self.model.clear();
        let mut root = ::std::collections::HashMap::new();

        for filter in filters {
            self.append(&mut root, filter);
        }

        self.filters.expand_all();

        for path in paths {
            self.filters.set_cursor(&path, None, false);
        }
    }

    fn append(&self, root: &mut ::std::collections::HashMap<String, ::gtk::TreeIter>, filter: (String, (u32, u32)))
    {
        use gtk::ToValue;
        use std::slice::SliceConcatExt;

        let (filter, (done, total)) = filter;
        let progress = (done as f32 / total as f32) * 100.;
        let f = filter.clone();

        let mut levels: Vec<_> = f.split('-')
            .collect();
        let title = levels.pop()
            .unwrap();

        let parent = levels.join("-");

        if !parent.is_empty() && root.get(&parent).is_none() {
            self.append(root, (parent.clone(), (0, 0)));
        }

        let row = self.model.append(root.get(&parent));

        self.model.set_value(&row, Column::Title.into(), &title.to_value());
        self.model.set_value(&row, Column::Raw.into(), &filter.to_value());
        self.model.set_value(&row, Column::Progress.into(), &progress.to_value());

        let tooltip = format!("{}/{}", done, total);
        self.model.set_value(&row, Column::Tooltip.into(), &tooltip.to_value());

        root.insert(filter, row);
    }

    fn update_tasks(&self, tasks: Vec<::tasks::Task>)
    {
        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }

    fn select_range(treeview: &::gtk::TreeView, path: &::gtk::TreePath)
    {
        let model = treeview.get_model()
            .unwrap();

        let start = path;
        let start_iter = model.get_iter(path)
            .unwrap();

        let n_child = model.iter_n_children(Some(&start_iter));

        if n_child > 0 {
            let end_iter = model.iter_nth_child(Some(&start_iter), n_child - 1)
                .unwrap();
            let end = model.get_path(&end_iter)
                .unwrap();

            treeview.get_selection()
                .select_range(start, &end);
        }
    }
}

#[widget]
impl ::relm::Widget for Filter
{
    fn init_view(&mut self)
    {
        self.filters.set_size_request(200, -1);
        self.scroll.set_policy(::gtk::PolicyType::Never, ::gtk::PolicyType::Automatic);
        self.filters.set_model(Some(&self.model));
        self.filters.get_selection().set_mode(::gtk::SelectionMode::Multiple);

        let column = ::gtk::TreeViewColumn::new();
        self.filters.append_column(&column);

        let cell = ::gtk::CellRendererProgress::new();
        cell.set_property_text_xalign(0.);
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", Column::Title.into());
        column.add_attribute(&cell, "value", Column::Progress.into());

        self.filters.set_tooltip_column(Column::Tooltip.into());
    }

    fn model(_: ()) -> ::gtk::TreeStore
    {
        let columns = vec![
            ::gtk::Type::String,
            ::gtk::Type::String,
            ::gtk::Type::U32,
            ::gtk::Type::String,
        ];

        ::gtk::TreeStore::new(&columns)
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Filters(_) => (),
            UpdateFilters(filters) => self.update_filters(filters),
            UpdateTasks(tasks) => self.update_tasks(tasks),
        }
    }

    view!
    {
        gtk::Paned {
            orientation: ::gtk::Orientation::Horizontal,
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
                        let (paths, list_model) = selection.get_selected_rows();

                        for path in paths {
                            let iter = match list_model.get_iter(&path) {
                                Some(iter) => iter,
                                None => continue,
                            };

                            match list_model.get_value(&iter, Column::Raw.into()).get() {
                                Some(value) => filters.push(value),
                                None => continue,
                            };
                        }

                        Msg::Filters(filters)
                    },
                }
            },
            gtk::ScrolledWindow {
                #[name="tasks"]
                ::widgets::Tasks {
                    Complete(ref task) => Msg::Complete(task.clone()),
                    Edit(ref task) => Msg::Edit(task.clone()),
                },
            }
        }
    }
}
