
#[derive(Debug, Clone)]
pub struct EffiEnvironment {
    pub todo_directory_path: String,
    pub todo_file_path: String,
    pub done_file_path: String
}

fn initialize_effi_file(file_path: &str){
    // If a todo.txt or done.txt file does not exist, create an empty one
    if !std::fs::metadata(file_path).is_ok(){
        log::info!("todo.txt or done.txt does not exist, creating an empty file at {}", file_path);
        match std::fs::File::create(file_path){
            Ok(_) => {},
            Err(err) => {
                log::error!("Unable to create {} because {}",file_path, err);
            },
        }
    }

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

        initialize_effi_file(&todo_file);
        initialize_effi_file(&done_file);

        Self{
            todo_directory_path: todo_dir,
            todo_file_path: todo_file,
            done_file_path: done_file
        }
    }

}