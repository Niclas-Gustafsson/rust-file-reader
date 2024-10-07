use std::{
    env::{self, current_exe},
    fs::{self, DirEntry},
    io::{BufRead, BufReader, Result},
    path::{Path, PathBuf},
    process,
};

use std::ffi::OsStr;

fn main() -> Result<()> {
    //Take in cmd args for folder to recursively go through reading the files and their lines.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No arguments given, quitting program.");
        process::exit(0);
    }

    let path_to_folder = &args[1];
    //let file_path = &args[2];
    let path = PathBuf::from(path_to_folder);
    let (files, lines_in_files) = read_folder(&path, &path)?;
    println!("Number of files read: {}", files);
    println!("Number of total lines read: {}", lines_in_files);

    Ok(())
}

fn read_folder(dir: &Path, current_path: &Path) -> Result<(u64, u64)> {
    let mut files = 0;
    let mut lines_in_files = 0;

    // given name of folder or file will be ignored. Should be a file to take in.
    let ignore_file = OsStr::new("nicignore.txt");

    if let Ok(entries) = fs::read_dir(current_path) {
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            //find out if check_ignored returns true, we don't care about if it's false.
            match check_ignored(&dir, &entry) {
                Ok(val) => {
                    if val {
                        continue;
                    } else {
                        ()
                    }
                }
                Err(e) => println!("{}", e),
            }

            // If ignore_file exists, skip reading it.
            if ignore_file == path.file_name().unwrap() {
                continue;
            }
            if path.is_dir() {
                //call self recursively
                let (sub_files, sub_lines) = read_folder(&dir, &path)?;
                files += sub_files;
                lines_in_files += sub_lines;
            } else if path.is_file() {
                // count files and lines
                println!("{:?}", path.file_name());
                files += 1;
                lines_in_files += read_file(&path)?;
            }
        }
    }
    Ok((files, lines_in_files))
}

fn read_file(path: &Path) -> Result<u64> {
    let mut lines = 0;
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.is_ok() {
            lines += 1;
        }
    }
    Ok(lines)
}

fn check_ignored(file: &Path, current_file: &DirEntry) -> Result<bool> {
    // Read ignore file, parse lines into vec, check file_name passed as arg against all lines in ignore vec. return boolean if true ignore file, else read it.
    let mut should_ignore: bool = false;
    let ignore_file_path = file.to_str();
    let ignore_filename = "\\nicignore.txt";

    let file = fs::File::open(ignore_file_path.unwrap().to_owned() + ignore_filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.is_ok() {
            if current_file.file_name().to_str()
                == Some(line.as_ref().unwrap().to_string().as_str())
            {
                println!(
                    "File exists in ignore file, the file is: {:?}",
                    current_file.file_name().to_str()
                );
                should_ignore = true;
            }
        }
    }

    Ok(should_ignore)
}
