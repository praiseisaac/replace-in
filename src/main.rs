use std::env;
use std::fs;
use std::path::Path;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        let mut directory: Vec<&str> = (&args[1]).split(":").collect();
        let placeholder = &args[2];
        let replacement = &args[3];
        let mut result = 0;
        let mut path: String = format!(
            "/{}",
            String::from(&directory[1..directory.len()].join("/"))
        );
        println!("Folder: {}\n\n", path);
        if Path::new(&path).exists() {
            println!(
                "Changing: \n\n---------\n\nFrom: {}\n\nTo: {}\n\nIn: {}\n\n---------\n\n",
                placeholder, replacement, &path,
            );
            if Path::new(&path).is_dir() {
                let last_dir = &directory.len() - 2;
                if &directory[last_dir] == placeholder {
                    directory[last_dir] = &replacement;
                    let replaced_path = format!(
                        "/{}",
                        String::from(&directory[1..directory.len()].join("/"))
                    );
                    fs::rename(
                        &path[..&path.len() - 1],
                        &replaced_path[..&replaced_path.len() - 1],
                    )
                    .expect(&path);
                    path = format!("{}/", &replaced_path);
                    result = 1;
                }
                let paths = fs::read_dir(&path).unwrap();
                result += recursive_print(paths, &placeholder, &replacement);
            } else if Path::new(&path).is_file() {
                replace(&path, &placeholder, &replacement, 0);
            }
            println!("Occurrences changed: {}", result);
        }
    }
}

fn recursive_print(paths: fs::ReadDir, string_to_replace: &str, replacement: &str) -> i64 {
    let mut res = 0;
    for path in paths {
        let path_name = path.unwrap().path().display().to_string();
        if Path::new(&path_name).exists() {
            if Path::new(&path_name).is_file() {
                res += replace(&path_name, &string_to_replace, &replacement, 0);
            } else if Path::new(&path_name).is_dir() {
                let current = fs::read_dir(&path_name).unwrap();
                res += recursive_print(current, &string_to_replace, &replacement);
            }
        }
    }
    return res;
}

fn replace(filename: &String, string_to_replace: &str, replacement: &str, mut result: i64) -> i64 {
    if Path::new(&filename).exists() {
        match fs::read_to_string(&filename) {
            Ok(contents) => {
                if contents.len() > 0 {
                    let replaced = str::replace(&contents, &string_to_replace, &replacement);
                    let occurrences = contents.matches(&string_to_replace).count();
                    if replaced != contents {
                        fs::write(&filename, &replaced).expect("could not write to file");
                        result += occurrences as i64;
                    }
                }
            }
            Err(_) => {}
        }
        if filename.contains(&string_to_replace) {
            rename(filename, &string_to_replace, &replacement);
            result += 1;
        }
    } else {
    }
    return result;
}

fn rename(filename: &str, string_to_replace: &str, replacement: &str) {
    let new_file: String = str::replace(&filename, &string_to_replace, &replacement);
    fs::rename(&filename, &new_file).expect("Could not rename file");
}
