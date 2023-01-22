
#[derive(Debug, Clone)]
pub struct EffiEnvironment {
    pub todo_directory_path: String,
    pub todo_file_path: String,
    pub done_file_path: String
}

impl EffiEnvironment {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir();
        let home_dir_path = match home_dir{
            Some(home_dir) => home_dir,
            None => std::process::exit(1),
        };
        let home_dir_str = match home_dir_path.as_path().to_str(){
            Some(str) => str,
            None => {
                std::process::exit(1);
            } 
        };

        let todo_dir = format!("{}{}", home_dir_str, "/.local/opt/share/todo");
        let todo_file = format!("{}/{}", todo_dir, "todo.txt");
        let done_file = format!("{}/{}", todo_dir, "done.txt");
        Self{
            todo_directory_path: todo_dir,
            todo_file_path: todo_file,
            done_file_path: done_file
        }
    }
}