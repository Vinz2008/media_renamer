use std::fs;
use std::vec;

// TODO : add remover of pattern
// example : filename is "series - 01 - ac3"
// media_renamer --remover ac3 .
// will output the filename without ac3 : "series - 01 - "

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
            //files_and_renamed_files.push((file_path, renamed_file));
            //panic!("not implemented yet");  
        }
        if let Some(append_season) = &args.mode.append_season {
            renamed_file = rename_file(&renamed_file, append_season.season_number);
            println!("new name : {}", renamed_file);
            //files_and_renamed_files.push((file_path, renamed_file));
        }
        files_and_renamed_files.push((file_path, renamed_file));
        /*match args.mode {
            Mode::AppendSeason(ref arg) => {
                let renamed_file = rename_file(&file_path, arg.season_number);
                println!("new name : {}", renamed_file);
                files_and_renamed_files.push((file_path, renamed_file));
            },
            Mode::PatternRemover => {}
        }*/
        //rename(file_path, renamed_file).unwrap();
    }

    for pairs in files_and_renamed_files {
        fs::rename(pairs.0, pairs.1).unwrap();
    } 
}

#[cfg(test)]
mod tests;