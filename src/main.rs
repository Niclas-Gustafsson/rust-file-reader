use std::{
    env, fs,
    io::{BufRead, BufReader, Result},
    path::{Path, PathBuf},
    process,
};
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
    let (files, lines_in_files) = read_folder(&path)?;
    println!("Number of files read: {}", files);
    println!("Number of total lines read: {}", lines_in_files);

    Ok(())
}

fn read_folder(dir: &Path) -> Result<(u64, u64)> {
    let mut files = 0;
    let mut lines_in_files = 0;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();

            println!("{:?}", entry.file_name());
            if path.is_dir() {
                //call self recursively
                let (sub_files, sub_lines) = read_folder(&path)?;
                files += sub_files;
                lines_in_files += sub_lines;
            } else if path.is_file() {
                // count files and lines
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
