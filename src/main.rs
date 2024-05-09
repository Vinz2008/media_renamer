use std::fs;

mod args;

use args::{handle_args, DashRemoverArgs};

mod parser;

use parser::{append_season, remove_pattern, dash_remover};

use std::env;

use crate::help::print_usage;

mod help;

fn get_max_len_nb_episode_env() -> Option<usize> {
    let max_len_nb_episode_env = env::var("MAX_LEN_NB_EPISODE");
    match max_len_nb_episode_env {
        Err(_) => { None }
        Ok(max_len_nb_episode_str) => {
            let max_len_nb_episode = max_len_nb_episode_str.parse::<usize>().unwrap();
            Some(max_len_nb_episode)
        }
    }
}

fn main() {
    let args = handle_args();
    if args.is_help_mode {
        print_usage();
        return;
    }
    println!("{}", args.folder_path);
    let max_len_nb_episode : Option<usize> = get_max_len_nb_episode_env();
    let paths = fs::read_dir(args.folder_path).unwrap();
    let mut files_and_renamed_files : Vec<(String, String)> = vec![];
    for path in paths {
        let file_path = path.unwrap().path().into_os_string().into_string().unwrap();
        println!("Name: {}", file_path);
        let mut renamed_file = file_path.clone();
        if let Some(pattern_remover) = &args.mode.pattern_remover {
            renamed_file = remove_pattern(&renamed_file, &pattern_remover.pattern);
        }
        if let Some(append_season_arg) = &args.mode.append_season {
            renamed_file = append_season(&renamed_file, append_season_arg.season_number, max_len_nb_episode);
        }
        if let Some(DashRemoverArgs) = args.mode.dash_remover {
            renamed_file = dash_remover(&renamed_file);
        }
        println!("new name : {}", renamed_file);
        files_and_renamed_files.push((file_path, renamed_file));
    }

    for pairs in files_and_renamed_files {
        fs::rename(pairs.0, pairs.1).unwrap();
    } 
}

#[cfg(test)]
mod tests;