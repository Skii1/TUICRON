use std::fs;
use std::io::Error;
use std::path::Path;

/*This is responsible for creating, editing, managing
the cron tasks data file. A Vector will be formed here,
which will be converted to a vec of "CronTasks", and handed to
app.tasks during runtime for adding tasks in the app. app.tasks
hands information to the file buffer vector, and writes to the
save file at the end of run time, or during a manual user induced
save file event.
 */
pub struct File {
    pub path: String,
    pub msg: String,
   // pub buff: vec![],
    pub exists: bool,
    pub exiting: bool,
}

impl File {
    pub fn new() -> File {
        File {
            path: String::from("./data"),
           // buff: Vec::new(),
            msg: String::new(),
            exists: false,
            exiting: false
        }
    }
    pub fn create_path(&mut self) {
        fs::create_dir(&self.path);
        let my_path: &Path = std::path::Path::new(&self.path);
        if my_path.exists() {
            self.msg = String::from("Found existing data file path.");
            return;
        }
        let create_dir_result: Result<(), Error> = fs::create_dir(&self.path);
        if create_dir_result.is_ok() {
            self.msg = String::from("New data directory created.")
        }
        else {
            self.msg = format!("An errror occured while creating the save data directory : {:?}", create_dir_result)
        }
    }

    pub fn make() {
        let path: &str = "./data/file.txt";
        let path2: &str = "./data/file1.txt";
        let text: &str = "Hello, world!!!";
        let text2: &str = "second file";
        _ = std::fs::write(path, text);
        _ = std::fs::write(path2, text2);

    }
    pub fn read() {}

    pub fn write() {}

    pub fn make_buffer() {}
}