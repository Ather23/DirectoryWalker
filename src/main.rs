use clap::Parser;
use colored::Colorize;
use std::{
    fs::{self},
    path::Path,
};

#[derive(Parser, Debug)]
#[command(author = "Ather", version)]
struct Args {
    #[arg(short, long)]
    dir_path: String,
}

fn main() {
    let args = Args::parse();
    walk_directory(&args.dir_path, 0);
}

fn build_level_symbol_string(mut n: i16) -> String {
    n *= 3;
    let mut dashes = String::new();
    let level_sym = "├─".to_string();
    dashes = "─".repeat(n as usize).to_string();
    let level = format!("{}{}", level_sym, dashes.to_string());
    level
}

fn remove_ancestors_from_path(full_path: &str) -> String {
    let sep = dir_seperator();

    full_path.split(sep).last().unwrap().to_string()
}

fn dir_seperator() -> char {
    if cfg!(target_os = "windows") {
        return '\\';
    } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        return '/';
    } else {
        panic!("Unsupported OS");
    }
}

fn walk_directory(path_string: &str, mut depth: i16) {
    let path = Path::new(path_string);
    let files = fs::read_dir(path).unwrap();
    let mut dirs: Vec<String> = Vec::new();
    let sep = dir_seperator();

    if depth == 0 {
        println!(".");
    }
    for file in files {
        match file {
            Ok(fs) => {
                let file_type = fs.file_type().unwrap();
                if file_type.is_file() {
                    let dash = build_level_symbol_string(depth);
                    println!(
                        "{} {}",
                        dash.cyan(),
                        fs.file_name().to_string_lossy().cyan()
                    );
                } else if file_type.is_dir() {
                    let dir_path = fs.path().display().to_string();
                    dirs.push(dir_path);
                }
            }
            Err(err) => {
                continue;
            }
        }
    }
    while let Some(dr) = dirs.pop() {
        let dash = build_level_symbol_string(depth);
        let folder_name = remove_ancestors_from_path(&dr);
        println!(
            "{}.{sep}{}",
            dash.yellow().bold(),
            folder_name.yellow().bold()
        );
        depth += 1;
        walk_directory(dr.as_str(), depth);
    }
}
