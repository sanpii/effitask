#![allow(deprecated)]

use gtk::prelude::*;

#[derive(Debug)]
pub enum MsgInput {
    Add,
    Delete,
    Edit(Column, gtk::TreePath, String),
    Set(std::collections::BTreeMap<String, String>),
}

#[derive(Debug)]
pub enum MsgOutput {
    Updated(std::collections::BTreeMap<String, String>),
}

pub struct Model {
    keywords: std::collections::BTreeMap<gtk::TreePath, (String, String)>,
    store: gtk::ListStore,
    tree_view: gtk::TreeView,
}

#[repr(u32)]
#[derive(Clone, Debug)]
pub enum Column {
    Name = 0,
    Value = 1,
}

impl From<Column> for u32 {
    fn from(column: Column) -> Self {
        unsafe { std::mem::transmute(column) }
    }
}

impl From<Column> for i32 {
    fn from(column: Column) -> Self {
        unsafe { std::mem::transmute(column) }
    }
}

impl Model {
    fn add(&mut self) {
        let iter = self.store.append();
        let path = self.store.path(&iter);
        let column = self.tree_view.column(Column::Name.into());

        self.keywords
            .insert(path.clone(), (String::new(), String::new()));
        gtk::prelude::TreeViewExt::set_cursor(&self.tree_view, &path, column.as_ref(), true);
    }

    fn delete(&mut self, sender: relm4::ComponentSender<Self>) {
        let selection = self.tree_view.selection();
        let (rows, _) = selection.selected_rows();
        let references = rows
            .iter()
            .map(|x| gtk::TreeRowReference::new(&self.store, x));

        for reference in references.flatten() {
            if let Some(path) = reference.path() {
                self.keywords.remove(&path);
                if let Some(iter) = self.store.iter(&path) {
                    self.store.remove(&iter);
                }
            }
        }

        sender.output(MsgOutput::Updated(self.keywords())).ok();
    }

    fn edit(
        &mut self,
        sender: relm4::ComponentSender<Self>,
        column: Column,
        path: &gtk::TreePath,
        new_text: &str,
    ) {
        if let Some(keyword) = self.keywords.get_mut(path) {
            match column {
                Column::Name => keyword.0 = new_text.to_string(),
                Column::Value => keyword.1 = new_text.to_string(),
            }
        }

        let iter = self.store.iter(path).unwrap();
        self.store
            .set_value(&iter, column.into(), &new_text.to_value());

        sender.output(MsgOutput::Updated(self.keywords())).ok();
    }

    fn keywords(&self) -> std::collections::BTreeMap<String, String> {
        self.keywords.values().cloned().collect()
    }

    fn set(&mut self, tags: std::collections::BTreeMap<String, String>) {
        self.keywords.clear();
        self.store.clear();

        for (name, value) in tags {
            let iter = self.store.append();

            self.store
                .set_value(&iter, Column::Name.into(), &name.to_value());
            self.store
                .set_value(&iter, Column::Value.into(), &value.to_value());

            let path = self.store.path(&iter);

            self.keywords.insert(path, (name, value));
        }
    }
}

#[relm4::component(pub)]
impl relm4::SimpleComponent for Model {
    type Init = std::collections::BTreeMap<String, String>;
    type Input = MsgInput;
    type Output = MsgOutput;

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        use gtk::glib::clone;

        let columns = vec![
            gtk::glib::types::Type::STRING,
            gtk::glib::types::Type::STRING,
        ];

        let model = Self {
            keywords: std::collections::BTreeMap::new(),
            store: gtk::ListStore::new(&columns),
            tree_view: gtk::TreeView::new(),
        };

        let tree_view = &model.tree_view;
        let widgets = view_output!();

        tree_view.set_model(Some(&model.store));
        tree_view.selection().set_mode(gtk::SelectionMode::Multiple);

        let column = gtk::TreeViewColumn::new();
        column.set_title("name");
        tree_view.append_column(&column);

        let cell = gtk::CellRendererText::new();
        cell.set_editable(true);
        cell.connect_edited(clone!(
            #[strong]
            sender,
            move |_, path, new_text| sender.input(MsgInput::Edit(
                Column::Name,
                path,
                new_text.to_string()
            ))
        ));

        gtk::prelude::CellLayoutExt::pack_start(&column, &cell, true);
        column.add_attribute(&cell, "text", Column::Name.into());

        let column = gtk::TreeViewColumn::new();
        column.set_title("value");
        tree_view.append_column(&column);

        let cell = gtk::CellRendererText::new();
        cell.set_editable(true);
        cell.connect_edited(clone!(
            #[strong]
            sender,
            move |_, path, new_text| sender.input(MsgInput::Edit(
                Column::Value,
                path,
                new_text.to_string()
            ))
        ));

        gtk::prelude::CellLayoutExt::pack_start(&column, &cell, true);
        column.add_attribute(&cell, "text", Column::Value.into());

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: relm4::ComponentSender<Self>) {
        use MsgInput::*;

        match msg {
            Add => self.add(),
            Delete => self.delete(sender),
            Edit(ref column, ref path, ref new_text) => {
                self.edit(sender, column.clone(), path, new_text)
            }
            Set(keywords) => self.set(keywords),
        }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::ScrolledWindow {
                set_height_request: 150,
                set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),

                #[local_ref]
                tree_view -> gtk::TreeView {
                    set_headers_visible: true,
                    set_hexpand: true,
                    set_vexpand: true,
                },
            },
            gtk::ActionBar {
                pack_start = &gtk::Button {
                    set_icon_name: "list-add",
                    connect_clicked => MsgInput::Add,
                },
                pack_start = &gtk::Button {
                    set_icon_name: "list-remove",
                    connect_clicked => MsgInput::Delete,
                },
            },
        },
    }
}
