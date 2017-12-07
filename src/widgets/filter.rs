use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Filter(Option<String>),
    UpdateFilters(Vec<(String, u32)>),
    UpdateTasks(Vec<::tasks::Task>),
}

#[repr(u32)]
enum Column {
    Title = 0,
    Raw = 1,
    Progress = 2,
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
    fn populate_filters(&mut self, filters: Vec<(String, u32)>)
    {
        self.model.clear();
        let mut root = ::std::collections::HashMap::new();

        for filter in filters {
            self.append(&mut root, filter);
        }

        self.filters.expand_all();
    }

    fn append(&self, root: &mut ::std::collections::HashMap<String, ::gtk::TreeIter>, filter: (String, u32))
    {
        use gtk::ToValue;
        use std::slice::SliceConcatExt;

        let (filter, progress) = filter;
        let f = filter.clone();

        let mut levels: Vec<_> = f.split("-")
            .collect();
        let title = levels.pop()
            .unwrap();

        let parent = levels.join("-");

        if parent.len() > 0 && root.get(&parent).is_none() {
            self.append(root, (parent.clone(), 0));
        }

        let row = self.model.append(root.get(&parent));

        self.model.set_value(&row, Column::Title.into(), &title.to_value());
        self.model.set_value(&row, Column::Raw.into(), &filter.to_value());
        self.model.set_value(&row, Column::Progress.into(), &progress.to_value());

        root.insert(filter, row);
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

        let column = ::gtk::TreeViewColumn::new();
        self.filters.append_column(&column);

        let cell = ::gtk::CellRendererProgress::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", Column::Title.into());
        column.add_attribute(&cell, "value", Column::Progress.into());
    }

    fn model(_: ()) -> ::gtk::TreeStore
    {
        let columns = vec![
            ::gtk::Type::String,
            ::gtk::Type::String,
            ::gtk::Type::U32,
        ];

        ::gtk::TreeStore::new(&columns)
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
            orientation: ::gtk::Orientation::Horizontal,
            #[name="scroll"]
            gtk::ScrolledWindow {
                #[name="filters"]
                gtk::TreeView {
                    headers_visible: false,
                    selection.changed(selection) => {
                        if let Some((list_model, iter)) = selection.get_selected() {
                            let filter = list_model.get_value(&iter, Column::Raw.into())
                                .get();

                            Msg::Filter(filter)
                        }
                        else {
                            Msg::Filter(None)
                        }
                    },
                }
            },
            gtk::ScrolledWindow {
                #[name="tasks"]
                ::widgets::Tasks,
            }
        }
    }
}
