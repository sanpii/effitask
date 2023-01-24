
#[derive(Debug, Clone)]
pub struct Environment {
    pub todo_dir: String,
    pub todo_file: String,
    pub done_file: String
}

fn initialize_effi_file(file_path: &str) -> Result<(), std::io::Error>{
    // If a todo.txt or done.txt file does not exist, create an empty one

    let file_path = std::path::Path::new(file_path);
    if !file_path.exists(){
        log::debug!("todo.txt or done.txt does not exist, creating an empty file at {}", file_path.display());
        match std::fs::File::create(file_path){
            Ok(_) => {
                return Ok(());
            },
            Err(err) => {
                log::debug!("Unable to create {} because {}",file_path.display(), err);
                return Err(err);
            },
        }
    }
    return Ok(());
}

fn initialize_effi_directory(directory_path: &str) -> Result<(), std::io::Error>{
    let directory_path = std::path::Path::new(directory_path);


    if !directory_path.exists(){
        match std::fs::create_dir_all(directory_path){
            Ok(_) => {
                return Ok(());
            },
            Err(err) =>  {
                log::debug!("Unable to create {:?} because {}", directory_path.display(), err);
                return Err(err);
            }
        }
    }
    return Ok(());
}

impl Environment {
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
        
        match initialize_effi_directory(&todo_dir){
            Ok(_) => {},
            Err(_) => {
                log::error!("Unable to initialize todo directory {}", todo_dir);
            },
        }
        match initialize_effi_file(&todo_file){
            Ok(_) => {},
            Err(_) => {
                // Should we panic here or just log something?
                log::error!("Unable to initialize {}", todo_file)
            },
        }
        match initialize_effi_file(&done_file){
            Ok(_) => {},
            Err(_) => {
                log::error!("Unable to initialize {}", done_file)
            },
        }

        Self{
            todo_dir,
            todo_file,
            done_file
        }
    }

}