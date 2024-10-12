use std::{env::args, process};

use crate::{
    create_path::PathInfo,
    crud_ops::{add_task, delete_task, edit_task, show_tasks},
};

pub fn read_args(path_info: &PathInfo) {
    let args: Vec<String> = args().collect();

    //if run without arguments. then run help
    if args.len() <= 1 {
        show_help();
        process::exit(0);
    } else if args.len() == 4 && args[1] != "-e" {
        println!("to much arguments for option {}", args[1]);
        process::exit(1);
    } else if args.len() > 4 {
        println!("to much arguments for option {}", args[1]);
        process::exit(1);
    }

    //counting for options
    let mut total_options = 0;
    for i in &args {
        if i.as_bytes()[0] == b'-' {
            total_options += 1;
        }
    }

    //check for multiple options
    if total_options > 1 {
        println!("cannot using multiple options");
        process::exit(1);
    }

    handlle_options(&args, path_info);
}

fn show_help() {
    println!("  options\n");

    println!("   -h | --help                {:>7}: show help", "");
    println!(
        "   -a | --add [task]          {:>7}: adding task into todo list",
        ""
    );
    println!(
        "   -d | --delete [number]     {:>7}: deleting task from todo list",
        ""
    );
    println!("   -l | --list                {:<7}: show all tasks", "");
    println!("   -e | --edit [number] [new] {:<7}: edit task", "");

    println!("\n\n")
}

fn handlle_options(args: &Vec<String>, path_info: &PathInfo) {
    match args[1].as_str() {
        "-h" | "--help" => {
            if is_exist(args, 2, false) == true {
                println!("option {} doesn't have arguments", args[1]);
                process::exit(1);
            }

            show_help();
        }
        "-a" | "--add" => {
            if is_exist(args, 2, true) == false {
                process::exit(1);
            };

            add_task(args[2].as_str(), path_info);
        }
        "-d" | "--delete" => {
            if is_exist(args, 2, true) == false {
                process::exit(1);
            };

            delete_task(args[2].as_str(), path_info);
        }
        "-l" | "--list" => {
            if is_exist(args, 2, false) == true {
                println!("option {} doesn't have arguments", args[1]);
                process::exit(1);
            }

            show_tasks(path_info);
        }
        "-e" | "--edit" => {
            if is_exist(args, 3, true) == false {
                process::exit(1);
            }

            edit_task(args[2].as_str(), args[3].as_str(), path_info);
        }
        e => {
            println!("unexpected {} option", e);
            process::exit(1);
        }
    }
}

fn is_exist(args: &Vec<String>, i: usize, show: bool) -> bool {
    match args.get(i) {
        Some(_) => true,
        None => {
            if show {
                println!("missing arguments for option `{}`", args[1]);
            }
            return false;
        }
    }
}
