use std::io;
use std::path::Path;
use std::fs;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

const DATA_DIRECTORY: &str = "data/uploads/"; /* make sure to include trailing slash */
const URL_SIZE: i32 = 5;

pub fn make_new_paste(paste_data: Vec<u8>) -> io::Result<String> {
    let current_dir = std::env::current_dir().expect("Failed to get the path of the current working directory.");

    println!("Current directory : {}", current_dir.to_str().unwrap());

    let data_directory = Path::new(DATA_DIRECTORY);

    // create the output directory if it doesn't already exist
    if !data_directory.exists() {
        match fs::create_dir_all(DATA_DIRECTORY) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to created data directory. Error : {}", e)
            }
        };
    }

    let file_path; /* owned, mutable reference to file path */
    let file_name;

    // Find a filename that doesn't already exist
    loop {
        let filename = generate_filename();
        if !data_directory.join(&filename).exists() {
            file_path = data_directory.join(&filename);
            file_name = filename;
            break;
        }
    }

    match fs::write(file_path, paste_data) {
        Ok(_) => { Ok(file_name) }
        Err(e) => { Err(e) }
    }
}


pub fn generate_filename() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(URL_SIZE as usize)
        .map(char::from)
        .collect()
}