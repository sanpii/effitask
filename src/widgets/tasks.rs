use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    Update(Vec<::tasks::Task>),
}

#[repr(u32)]
enum Column {
    Finished = 0,
    Subject = 1,
    Color = 2,
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
    pub fn update(&mut self, tasks: Vec<::tasks::Task>)
    {
        use ::gtk::ToValue;

        self.model.clear();

        if tasks.is_empty() {
            self.tree_view.hide();
            self.label.show();
        }
        else {
            self.tree_view.show();
            self.label.hide();

            for task in tasks.iter() {
                let row = self.model.append();
                let color = self.task_color(task);

                self.model.set_value(&row, Column::Finished.into(), &task.finished.to_value());
                self.model.set_value(&row, Column::Subject.into(), &task.subject.to_value());
                self.model.set_value(&row, Column::Color.into(), &color.to_value());
            }
        }
    }

    fn task_color(&self, task: &::tasks::Task) -> &str
    {
        match task.priority {
            0 => "#F8D7DA",
            1 => "#FFF3CD",
            2 => "#D1ECF1",
            3 => "#D4EDDA",
            4 => "#E7E8EA",
            _ => "#FFFFFF",
        }
    }
}

#[widget]
impl ::relm::Widget for Tasks
{
    fn init_view(&mut self)
    {
        self.tree_view.set_model(Some(&self.model));

        let column = ::gtk::TreeViewColumn::new();
        self.tree_view.append_column(&column);

        let cell = ::gtk::CellRendererToggle::new();
        column.pack_start(&cell, false);
        column.add_attribute(&cell, "active", Column::Finished.into());
        column.add_attribute(&cell, "cell-background", Column::Color.into());

        let cell = ::gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "strikethrough", Column::Finished.into());
        column.add_attribute(&cell, "text", Column::Subject.into());
        column.add_attribute(&cell, "background", Column::Color.into());
    }

    fn model(_: ()) -> ::gtk::ListStore
    {
        let columns = vec![
            ::gtk::Type::Bool,
            ::gtk::Type::String,
            ::gtk::Type::String,
        ];

        ::gtk::ListStore::new(&columns)
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Update(tasks) => self.update(tasks),
        }
    }

    view!
    {
        gtk::Box {
            #[name="tree_view"]
            gtk::TreeView {
                padding: {
                    fill: true,
                    expand: true,
                },
                headers_visible: false,
            },
            #[name="label"]
            gtk::Label {
                padding: {
                    fill: true,
                    expand: true,
                },
                text: "Nothing to do :)",
            },
        }
    }
}
