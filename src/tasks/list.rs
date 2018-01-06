#[derive(Clone, Debug)]
pub struct List {
    pub tasks: Vec<::tasks::Task>,
    todo: String,
    done: String,
}

impl List
{
    pub fn new() -> Self
    {
        Self {
            tasks: Vec::new(),
            todo: String::new(),
            done: String::new(),
        }
    }

    pub fn from_files(todo: &str, done: &str) -> Self
    {
        let mut list = Self::new();

        list.load_todo(todo);
        list.load_done(done);

        list
    }

    fn load_todo(&mut self, todo: &str)
    {
        let tasks = self.load_file(todo);

        self.todo = todo.to_string();
        self.tasks.extend(tasks);
    }

    fn load_done(&mut self, done: &str)
    {
        let tasks = self.load_file(done);

        self.done = done.to_string();
        self.tasks.extend(tasks);
    }

    fn load_file(&self, path: &str) -> Vec<::tasks::Task>
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

        let last_id = self.tasks.len();

        for (id, line) in ::std::io::BufReader::new(file).lines().enumerate() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            match ::tasks::Task::from_str(line.as_str()) {
                Ok(mut task) => {
                    task.id = last_id + id;
                    tasks.push(task);
                },
                Err(_) => error!("Invalid tasks: '{}'", line),
            };
        }

        tasks
    }

    pub fn projects(&self) -> Vec<String>
    {
        let today = ::date::today();

        let mut projects = self.tasks.iter()
            .filter(|x| !x.finished && (x.threshold_date.is_none() || x.threshold_date.unwrap() <= today))
            .fold(Vec::new(), |mut acc, item| {
                let mut projects = item.projects.clone();

                acc.append(&mut projects);

                acc
            });

        projects.sort();
        projects.dedup();

        projects
    }

    pub fn contexts(&self) -> Vec<String>
    {
        let today = ::date::today();

        let mut contexts = self.tasks.iter()
            .filter(|x| !x.finished && (x.threshold_date.is_none() || x.threshold_date.unwrap() <= today))
            .fold(Vec::new(), |mut acc, item| {
                let mut contexts = item.contexts.clone();

                acc.append(&mut contexts);

                acc
            });

        contexts.sort();
        contexts.dedup();

        contexts
    }

    pub fn write(&self) -> Result<(), String>
    {
        let todo = self.tasks.iter()
            .filter(|x| !x.finished)
            .cloned()
            .collect();
        self.write_tasks(&self.todo, todo)?;

        let done = self.tasks.iter()
            .filter(|x| x.finished)
            .cloned()
            .collect();
        self.write_tasks(&self.done, done)?;

        Ok(())
    }

    fn write_tasks(&self, file: &str, tasks: Vec<::tasks::Task>) -> Result<(), String>
    {
        use std::io::Write;

        self.backup(file)?;

        let mut f = match ::std::fs::File::create(file) {
            Ok(f) => f,
            Err(err) => return Err(format!("Unable to write tasks: {}", err)),
        };

        for mut task in tasks {
            task.note = task.note.write()?;

            match f.write(format!("{}\n", task).as_bytes()) {
                Ok(_) => (),
                Err(err) => return Err(format!("Unable to write tasks: {}", err)),
            };
        }

        Ok(())
    }

    fn backup(&self, file: &str) -> Result<(), String>
    {
        let bak = format!("{}.bak", file);

        match ::std::fs::copy(file, bak) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unable to backup {}", file)),
        }
    }

    pub fn add(&mut self, text: &str) -> Result<(), String>
    {
        use std::str::FromStr;

        let task = match ::tasks::Task::from_str(text) {
            Ok(task) => task,
            Err(_) => return Err(format!("Unable to convert task: '{}'", text)),
        };

        self.append(task);
        self.write()
    }

    pub fn append(&mut self, task: ::tasks::Task)
    {
        self.tasks.push(task);
    }
}
