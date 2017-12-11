#[derive(Clone, Debug)]
pub struct Task {
    inner: ::todo_txt::Task,
    pub id: usize,
    pub note: Option<String>,
}

impl Task
{
    fn get_note(task: &::todo_txt::Task) -> Option<String>
    {
        let tag = match ::std::env::var("TODO_NOTE_TAG") {
            Ok(tag) => tag,
            Err(_) => "note".to_owned(),
        };

        if let Some(file) = task.tags.get(&tag) {
            Some(Self::load_note(file))
        }
        else {
            None
        }
    }

    fn load_note(file: &String) -> String
    {
        use std::io::Read;

        let mut note = String::new();

        let todo_dir = match ::std::env::var("TODO_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(_) => {
                error!("Launch this program via todo.sh");
                return note;
            },
        };

        let note_dir = match ::std::env::var("TODO_NOTES_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(_) => format!("{}/notes", todo_dir),
        };

        let note_file = format!("{}/{}", note_dir, file);

        let file = match ::std::fs::File::open(note_file.clone()) {
            Ok(file) => file,
            Err(_) => {
                error!("Unable to open {:?}", note_file);
                return note;
            },
        };

        let mut buffer = ::std::io::BufReader::new(file);

        match buffer.read_to_string(&mut note) {
            Ok(_) => (),
            Err(_) => {
                error!("Unable to read {:?}", note_file);
                return note;
            },
        };

        note
    }

    pub fn complete(&mut self)
    {
        let today = ::chrono::Local::now()
            .date()
            .naive_local();

        self.finished = true;
        self.finish_date = Some(today);
    }

    pub fn uncomplete(&mut self)
    {
        self.finished = false;
        self.finish_date = None;
    }
}

impl ::std::str::FromStr for Task
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()>
    {
        let mut task = ::todo_txt::Task::from_str(s)?;

        let note = Self::get_note(&task);
        task.tags.remove(&"note".to_owned());

        Ok(Self {
            id: 0,
            note: note,
            inner: task,
        })
    }
}

impl ::std::ops::Deref for Task
{
    type Target = ::todo_txt::Task;

    fn deref(&self) -> &Self::Target
    {
        &self.inner
    }
}

impl ::std::ops::DerefMut for Task
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.inner
    }
}

impl ::std::fmt::Display for Task
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
    {
        use std::ops::Deref;

        f.write_str(format!("{}", self.deref()).as_str())
    }
}
