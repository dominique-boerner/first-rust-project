use crate::i18n_file_reader::{I18nFile, I18nFileReader};

mod i18n_file_reader;

/// Entry point of the application
pub fn main() {
    let i18n_folder_path = String::from("i18n");
    let path = String::from("de.json");
    let i18n_file = I18nFile { path, i18n_folder_path};

    if !i18n_file.file_exists() {
        i18n_file.initialize_file();
    }

    let result = i18n_file.read_file();
    println!("{:?}", result)
}
