use std::{env::var, path::PathBuf, process};

use create_path::{Error, PathInfo};

mod create_path;
mod crud_ops;
mod date_time;
mod read_args;

fn main() {
    let path_info = PathInfo::new(if cfg!(target_os = "windows") {
        PathBuf::from(var("APPDATA").unwrap()).join("rstdo")
    } else if cfg!(target_os = "linux") {
        PathBuf::from(var("HOME").unwrap()).join("rstdo")
    } else if cfg!(target_os = "macos") {
        PathBuf::from(var("HOME").unwrap()).join("rstdo")
    } else {
        println!("error {:?}", Error::UNSUPPORTED_OS);
        process::exit(1);
    });

    read_args::read_args(&path_info);
    create_path::create_path(&path_info);
}
