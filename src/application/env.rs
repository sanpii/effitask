#[derive(Debug, Clone)]
pub struct Environment {
    pub todo_dir: String,
    pub todo_file: String,
    pub done_file: String,
}

fn initialize_effi_file(file_path: &str) -> Result<(), std::io::Error> {
    // If a todo.txt or done.txt file does not exist, create an empty one

    let file_path = std::path::Path::new(file_path);
    if !file_path.exists() {
        log::debug!(
            "todo.txt or done.txt does not exist, creating an empty file at {}",
            file_path.display()
        );
        match std::fs::File::create(file_path) {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                log::debug!("Unable to create {} because {}", file_path.display(), err);
                return Err(err);
            }
        }
    }
    return Ok(());
}

fn initialize_effi_directory(directory_path: &str) -> Result<(), std::io::Error> {
    let directory_path = std::path::Path::new(directory_path);

    if !directory_path.exists() {
        match std::fs::create_dir_all(directory_path) {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                log::debug!(
                    "Unable to create {:?} because {}",
                    directory_path.display(),
                    err
                );
                return Err(err);
            }
        }
    }
    return Ok(());
}

fn get_default_todo_dir() -> Result<String, std::io::Error> {
    let home_dir = dirs::home_dir();
    let home_dir_path = match home_dir {
        Some(home_dir) => home_dir,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unable to retrieve the user's home directory",
            ))
        }
    };

    let home_dir_str = match home_dir_path.as_path().to_str() {
        Some(str) => str,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unable to convert path to string",
            ))
        }
    };
    return Ok(format!("{home_dir_str}/.local/opt/share/todo"));
}

impl Environment {
    pub fn new() -> Result<Self, std::io::Error> {
        let todo_dir = match std::env::var("TODO_DIR") {
            Ok(val) => val,
            Err(_) => {
                let default_todo_dir = get_default_todo_dir()?;
                default_todo_dir
            }
        };

        let todo_file = format!("{todo_dir}/todo.txt");
        let done_file = format!("{todo_dir}/done.txt");

        match initialize_effi_directory(&todo_dir) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
        match initialize_effi_file(&todo_file) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
        match initialize_effi_file(&done_file) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        Ok(Self {
            todo_dir,
            todo_file,
            done_file,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_effi_file() {
        let test_file = "./test_file.txt";
        std::fs::remove_file(test_file).unwrap();

        // Test file does not exist
        let result = initialize_effi_file(test_file);
        assert!(result.is_ok());

        // Test file already exists
        let result = initialize_effi_file(test_file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_initialize_effi_directory() {
        let test_dir = "./test_dir";
        std::fs::remove_dir(test_dir).unwrap();

        // Test directory does not exist
        let result = initialize_effi_directory(test_dir);
        assert!(result.is_ok());

        // Test directory already exists
        let result = initialize_effi_directory(test_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_default_todo_dir() {
        // Test home directory can be retrieved
        let result = get_default_todo_dir();
        assert!(result.is_ok());
    }

    #[test]
    fn test_new() {
        // Test with TODO_DIR environment variable set
        std::env::set_var("TODO_DIR", "/tmp/todo");
        let environment = Environment::new();
        assert!(environment.is_ok());

        // Test without TODO_DIR environment variable set
        std::env::remove_var("TODO_DIR");
        let binding = dirs::home_dir().unwrap();
        let home_dir = binding.to_str().unwrap();
        let default_todo_dir = format!("{}/.local/opt/share/todo", home_dir);
        let environment = Environment::new();
        assert_eq!(environment.unwrap().todo_dir, default_todo_dir);
    }

    #[test]
    fn test_new_non_default_todo_dir_files_set() {
        // Test with TODO_DIR environment variable set
        std::env::set_var("TODO_DIR", "/tmp/todo");
        let environment = Environment::new();
        assert!(environment.is_ok());

        let environment = environment.unwrap();
        assert_eq!(environment.todo_file, "/tmp/todo/todo.txt");
        assert_eq!(environment.done_file, "/tmp/todo/done.txt");
    }
}
