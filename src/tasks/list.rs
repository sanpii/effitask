#[derive(Clone)]
pub struct List {
    pub todo: Vec<::todo_txt::Task>,
    pub done: Vec<::todo_txt::Task>,
}

impl List
{
    pub fn from_files(todo: &::std::path::Path, done: &::std::path::Path) -> Self
    {
        Self {
            todo: Self::load_file(todo),
            done: Self::load_file(done),
        }
    }

    fn load_file(path: &::std::path::Path) -> Vec<::todo_txt::Task>
    {
        use std::io::BufRead;
        use std::str::FromStr;

        let mut tasks = Vec::new();
        let file = match ::std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => {
                error!("Unable to open {:?}", path);

                return tasks;
            },
        };

        for line in ::std::io::BufReader::new(file).lines() {
            let line = line.unwrap();

            match ::todo_txt::Task::from_str(line.as_str()) {
                Ok(task) => tasks.push(task),
                Err(_) => error!("Invalid tasks: '{}'", line),
            };
        }

        tasks
    }

    pub fn projects(&self) -> Vec<String>
    {
        let mut projects = self.todo.iter().fold(Vec::new(), |mut acc, ref item| {
            acc.append(&mut item.projects.clone());

            acc
        });

        projects.sort();
        projects.dedup();

        projects
    }
}
