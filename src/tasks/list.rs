#[derive(Clone, Debug, Default)]
pub struct List {
    pub inner: todo_txt::task::List<super::Task>,
    todo: String,
    done: String,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_files(todo: &str, done: &str) -> Self {
        let mut list = Self::new();

        list.load_todo(todo);
        list.load_done(done);

        list
    }

    fn load_todo(&mut self, todo: &str) {
        let tasks = self.load_file(todo);

        self.todo = todo.to_string();
        self.inner.extend(tasks);
    }

    fn load_done(&mut self, done: &str) {
        let tasks = self.load_file(done);

        self.done = done.to_string();
        self.inner.extend(tasks);
    }

    fn load_file(&self, path: &str) -> Vec<crate::tasks::Task> {
        use std::io::BufRead;

        let mut tasks = Vec::new();
        let Ok(file) = std::fs::File::open(path) else {
            log::error!("Unable to open {path:?}");

            return tasks;
        };

        let last_id = self.inner.len();

        for (id, line) in std::io::BufReader::new(file).lines().enumerate() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let mut task = crate::tasks::Task::from(line);
            task.id = last_id + id;
            tasks.push(task);
        }

        tasks
    }

    pub fn projects(&self) -> Vec<String> {
        let today = crate::date::today();

        self.inner
            .iter()
            .filter(|x| {
                !x.finished && (x.threshold_date.is_none() || x.threshold_date.unwrap() <= today)
            })
            .collect::<todo_txt::task::List<_>>()
            .projects()
    }

    pub fn contexts(&self) -> Vec<String> {
        let today = crate::date::today();

        self.inner
            .iter()
            .filter(|x| {
                !x.finished && (x.threshold_date.is_none() || x.threshold_date.unwrap() <= today)
            })
            .collect::<todo_txt::task::List<_>>()
            .contexts()
    }

    pub fn write(&self) -> Result<(), String> {
        let todo = self.inner.iter().filter(|x| !x.finished).cloned().collect();
        self.write_tasks(&self.todo, todo)?;

        let done = self.inner.iter().filter(|x| x.finished).cloned().collect();
        self.write_tasks(&self.done, done)?;

        Ok(())
    }

    fn write_tasks(&self, file: &str, tasks: Vec<crate::tasks::Task>) -> Result<(), String> {
        use std::io::Write;

        self.backup(file)?;

        let mut f = match std::fs::File::create(file) {
            Ok(f) => f,
            Err(err) => return Err(format!("Unable to write tasks: {err}")),
        };

        for mut task in tasks {
            if let Err(err) = task.note.write() {
                log::error!("Unable to save note: {err}");
                task.note = todo_txt::task::Note::None;
            }

            match f.write_all(format!("{task}\n").as_bytes()) {
                Ok(_) => (),
                Err(err) => log::error!("Unable to write tasks: {err}"),
            };
        }

        Ok(())
    }

    fn backup(&self, file: &str) -> Result<(), String> {
        let bak = format!("{file}.bak");

        match std::fs::copy(file, bak) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unable to backup {file}")),
        }
    }

    pub fn add(&mut self, text: &str) -> Result<(), String> {
        use std::str::FromStr as _;

        let mut task = crate::tasks::Task::from_str(text)
            .map_err(|_| format!("Unable to convert task: '{text}'"))?;

        task.create_date = Some(crate::date::today());

        self.append(task);
        self.write()
    }

    pub fn append(&mut self, task: crate::tasks::Task) {
        self.inner.push(task);
    }
}

impl std::ops::Deref for List {
    type Target = todo_txt::task::List<super::Task>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
