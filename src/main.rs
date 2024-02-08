use std::fs;
use std::vec;

mod args;

use args::handle_args;

mod parser;

use parser::{rename_file, remove_pattern};

fn main() {
    let args = handle_args();
    println!("{}", args.folder_path);
    let paths = fs::read_dir(args.folder_path).unwrap();
    let mut files_and_renamed_files : Vec<(String, String)> = vec![];
    for path in paths {
        let file_path = path.unwrap().path().into_os_string().into_string().unwrap();
        println!("Name: {}", file_path);
        let mut renamed_file = file_path.clone();
        if let Some(pattern_remover) = &args.mode.pattern_remover {
            renamed_file = remove_pattern(&renamed_file, &pattern_remover.pattern);
        }
        if let Some(append_season) = &args.mode.append_season {
            renamed_file = rename_file(&renamed_file, append_season.season_number);
            println!("new name : {}", renamed_file);
        }
        files_and_renamed_files.push((file_path, renamed_file));
    }

    for pairs in files_and_renamed_files {
        fs::rename(pairs.0, pairs.1).unwrap();
    } 
}

#[cfg(test)]
mod tests;