use std::fs::File;
use std::{fs, io};
use std::path::Path;
use serde_json::{Value};

pub trait I18nFileReader {
    fn create_file(&self) -> io::Result<File>;

    fn read_file(&self) -> Value;

    fn initialize_file(&self) ;

    fn write_json(&self, content: String);

    fn file_exists(&self) -> bool;

    fn create_i18n_folder(&self);

    fn get_file_path(&self) -> String;
}

pub struct I18nFile {
    pub path: String,
    pub i18n_folder_path: String,
}

/// This reader contains logic for reading and
/// writing an 18n json file.
impl I18nFileReader for I18nFile {

    /// create a i18n file.
    fn create_file(&self) -> io::Result<File> {
        return File::create(&self.path);
    }

    /// read a 18n file.
    fn read_file(&self) -> Value {
        let file_content = fs::read_to_string(&self.get_file_path()).expect("Unable to read file");
        return serde_json::from_str(&file_content).expect("Unable to parse file.");
    }

    /// initialize a i18n JSON file.
    fn initialize_file(&self) {
        self.create_i18n_folder();
        let initial_content = String::from("{}");
        self.create_file().expect("Cant create i18n file.");
        self.write_json(initial_content);
    }

    /// write json to file.
    fn write_json(&self, content: String) {
        fs::write(&self.get_file_path(), content).expect("Unable to write to file.");
    }

    /// checks, if a file exist on the path.
    fn file_exists(&self) -> bool {
        return Path::new(&self.get_file_path()).exists();
    }

    fn create_i18n_folder(&self) {
        fs::create_dir(&self.i18n_folder_path).expect("I18n Folder cant be created.");
    }

    fn get_file_path(&self) -> String {
        return self.i18n_folder_path.to_owned() + "/" + &self.path;
    }
}