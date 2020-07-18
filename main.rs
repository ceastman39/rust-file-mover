// Launches without console.
#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::fs::metadata;
use std::path::Path;
use std::io::prelude::*;

fn main() {

    // First, get arguments.
    let a: Vec<String> = env::args().collect();
    let mut out_path = String::new();
    let mut files_to_move: Vec<String> = Vec::new();

    // If launched with no or only one argument(s) given,
    // just open explorer to the path given for the output path.
    // (Explorer opens to Documents by default if no argument provided.)
    if a.len() <= 2 {

        let _child = match std::process::Command::new("explorer")
                                .arg(&a[1])
                                .spawn() {
                Ok(a) => a,
                Err(e) => {
                    write_str_to_file(format!("Error: {}", e));
                    panic!(format!("Error: {}", e));
                },
        };
        return;
    }
    // From 1, since 0 is ususally the filepath to the executable.
    for i in 1..a.len() {
        if i == 1 {
            out_path = String::from(&a[1]);
            let md = metadata(&out_path).unwrap();
            if !md.is_dir() {
                write_str_to_file(format!("Provided output path is not a directory: {}", out_path));
                panic!("Provided output path is not a directory: {}", out_path);
            }

        } else if i > 1 {
            files_to_move.push(String::from(&a[i]));
        };
    }

    for f_str in files_to_move {
        let from_path: &Path = Path::new(&f_str);
        if !from_path.exists() {
            write_str_to_file(format!("One of the file paths provided does not exist: {}", f_str));
            panic!("One of the file paths provided does not exist: {}", f_str);
        }
        let file_stem: &str = match from_path.file_stem() {
            Some(g) => match g.to_str() {
                Some(h) => h,
                None => {
                    write_str_to_file(format!("No file name provided in arg: {}", f_str));
                    panic!("No file name provided in arg: {}", f_str);
                },
            },
            None => {
                write_str_to_file(format!("No file name provided in arg: {}", f_str));
                panic!("No file name provided in arg: {}", f_str);
            },
        };

        let file_ext: &str = match from_path.extension() {
            Some(g) => match g.to_str() {
                Some(h) => h,
                None => {
                    write_str_to_file(format!("Unable to convert file extension to string, from {}", f_str));
                    panic!("Unable to convert file extension to string, from {}", f_str);
                },
            },
            None => "",
        };

        //Combine the whole filepath.
        let mut to_path = format!("{}\\{}.{}", &out_path, file_stem, file_ext);

        //Find a valid file name.
        let mut temp_path = Path::new(&to_path);
        if temp_path.exists() {
            let mut path_already_exists: bool = temp_path.exists();
            let mut i = 0;
            while path_already_exists {
                i += 1;
                let new_file_stem = &format!("{}_{}", file_stem, i);
                to_path = format!("{}\\{}.{}", &out_path, new_file_stem, file_ext);
                temp_path = Path::new(&to_path);
                path_already_exists = temp_path.exists();
            }
        }

        //Finally, copy the file to the specified path, and delete the original
        //Done this way since you can't "rename" a file to a different drive.
        match fs::copy(&f_str, &to_path) {
            Ok(_) => (),
            Err(e) => {
                write_str_to_file(format!("Error in moving file: {} From: {} To: {}", e, f_str, to_path));
                panic!("Error in moving file: {} From: {} To:{}", e, f_str, to_path);
            },
        }

        //Then, remove the original file, if it still exists.
        let temp_path = std::path::Path::new(&f_str);
        if temp_path.exists() {
            match fs::remove_file(&f_str) {
                Err(e) => {
                    write_str_to_file(format!("Error in deleting file: {} File: {}", e, f_str));
                    panic!("Error in deleting file: {} File: {}", e, f_str);
                },
                Ok(()) => (),
            };
        }

        write_str_to_file(format!("Successfully moved file from '{}' to '{}'.", f_str, to_path));
    }
}

// Used to log errors.
fn write_str_to_file(s: String) {

    let mut file = match OpenOptions::new().create(true).append(true)
    .open("log.txt")
    {
        Ok(f) => f,
        Err(e) => panic!("Oh god oh fuck: {}", e),
    };

    match writeln!(file, "{}", s) {
        Ok(_) => (),
        Err(e) => panic!("Error writing line: {}", e)
    };
}
