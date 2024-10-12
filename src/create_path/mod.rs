use std::{
    // env::var,
    fs::{DirBuilder, File},
    path::PathBuf,
    process,
};

pub fn create_path(path_info: &PathInfo){

    DirBuilder::new()
        .recursive(true)
        .create(&path_info.get_folder_path())
        .unwrap_or_default();

    if !path_info.get_folder_path().exists() {
        match DirBuilder::new()
            .recursive(true)
            .create(&path_info.get_folder_path())
        {
            Ok(_) => println!(
                "Created New Directory In: `{}`",
                path_info.get_folder_path().to_str().unwrap()
            ),
            Err(e) => match e.kind() {
                std::io::ErrorKind::OutOfMemory => {
                    println!("Not Enough Space For Writing New Directory");
                }
                e => {
                    println!("UnhandledError: {}", e);
                    process::exit(1);
                }
            },
        }
    }

    // file_path = folder_path.join("data.txt");

    match File::create_new(&path_info.get_file_path()) {
        Ok(_) => println!(
            "Created New file In: `{}`",
            path_info.get_file_path().to_str().unwrap()
        ),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            std::io::ErrorKind::OutOfMemory => {
                println!("Not Enough Space For Writing New File");
            }
            e => {
                println!("UnhandledError: {}", e);
                process::exit(1);
            }
        },
    };

}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Error {
    UNSUPPORTED_OS,
}

pub struct PathInfo {
    folder_path: PathBuf,
    file_path: PathBuf,
}

impl PathInfo {
    pub fn new(path: PathBuf) -> PathInfo {
        return PathInfo {
            folder_path: path.clone(),
            file_path: path.join("data.txt"),
        };
    }

    pub fn get_file_path(&self) -> PathBuf {
        return self.file_path.clone();
    }

    pub fn get_folder_path(&self) -> PathBuf {
        return self.folder_path.clone();
    }
}
