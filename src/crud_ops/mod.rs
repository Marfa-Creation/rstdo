use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process;

use crate::create_path::PathInfo;

pub fn edit_task(position: &str, new: &str, path_info: &PathInfo) {
    let position = position.parse::<usize>().unwrap_or_else(|_| {
        println!("arguments must be a number");
        process::exit(1);
    }) - 1;

    let temp_string = read_tasks_data(path_info);

    let mut data = VecDeque::from(temp_string.split("\n").collect::<Vec<&str>>());

    //clear empty string
    if data.len() != 0 {
        for i in 0..data.len() {
            if data[i] == "" {
                data.remove(i);
            }
        }
    }

    match data.get(position) {
        Some(_) => {
            data.remove(position).unwrap();
            data.insert(position, new);
        }
        None => {
            println!("no task at position: {}", position + 1);
            process::exit(1);
        }
    }

    let to_string = Vec::from(data).join("\n");

    if let Err(e) = fs::write(path_info.get_file_path(), to_string + "\n") {
        println!("error while writing file!\nerror: {}", e);
    }
}

pub fn delete_task(position: &str, path_info: &PathInfo) {
    let position = position.parse::<usize>().unwrap_or_else(|_| {
        println!("arguments must be a number");
        process::exit(1);
    }) - 1;
    let temp_string = read_tasks_data(path_info);

    let mut data = VecDeque::from(temp_string.as_str().split("\n").collect::<Vec<&str>>());

    //clear empty string
    if data.len() != 0 {
        for i in 0..data.len() {
            if data[i] == "" {
                data.remove(i);
            }
        }
    }

    data.remove(position).unwrap_or_else(|| {
        println!("no task at position: {}", position + 1);
        process::exit(1);
    });

    let to_string = Vec::from(data).join("\n");

    let new_line = if to_string.is_empty() { "" } else { "\n" };

    if let Err(e) = fs::write(path_info.get_file_path(), to_string + new_line) {
        println!("error while writing file!\nerror: {}", e);
    }
}

pub fn add_task(task: &str, path_info: &PathInfo) {
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(path_info.get_file_path())
        .unwrap();

    if let Err(e) = write!(file, "{}\n", task,) {
        println!("error while writing file\nerror: {}", e);
    };
}

pub fn show_tasks(path_info: &PathInfo) {
    if read_tasks_data(path_info).is_empty() {
        println!("todo list is empty");
        process::exit(1);
    }

    let temp_string = read_tasks_data(path_info);

    let mut iterable = temp_string.split("\n").collect::<Vec<&str>>();

    iterable.remove(iterable.len() - 1);

    for (index, item) in iterable.iter().enumerate() {
        println!("{}. {}", index + 1, item);
    }
}

pub fn read_tasks_data(path_info: &PathInfo) -> String {
    let file = fs::read_to_string(path_info.get_file_path()).unwrap_or_else(|e| {
        println!("error while reading file\nerror: {}", e);
        process::exit(1);
    });

    return file;
}
