#[derive(Clone)]
pub struct List {
    pub tasks: Vec<::tasks::Task>,
}

impl List
{
    pub fn from_files(todo: &::std::path::Path, done: &::std::path::Path) -> Self
    {
        let mut tasks = Vec::new();
        tasks.extend(Self::load_file(todo));
        tasks.extend(Self::load_file(done));

        Self {
            tasks,
        }
    }

    fn load_file(path: &::std::path::Path) -> Vec<::tasks::Task>
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

            match ::tasks::Task::from_str(line.as_str()) {
                Ok(task) => tasks.push(task),
                Err(_) => error!("Invalid tasks: '{}'", line),
            };
        }

        tasks
    }

    pub fn projects(&self) -> Vec<String>
    {
        let mut projects = self.tasks.iter().fold(Vec::new(), |mut acc, ref item| {
            acc.append(&mut item.projects.clone());

            acc
        });

        projects.sort();
        projects.dedup();

        projects
    }

    pub fn contexts(&self) -> Vec<String>
    {
        let mut contexts = self.tasks.iter().fold(Vec::new(), |mut acc, ref item| {
            acc.append(&mut item.contexts.clone());

            acc
        });

        contexts.sort();
        contexts.dedup();

        contexts
    }
}
