use std::env;

pub struct AppendSeasonArgs {
    pub season_number : i32
}

pub struct PatternRemoverArgs {
    pub pattern : String
}


// maybe use a vec of modes instead, but it need to be sorted first
pub struct Mode {
    pub append_season : Option<AppendSeasonArgs>, // default mode
    pub pattern_remover : Option<PatternRemoverArgs>,
}

pub struct Args {
    pub folder_path : String,
    pub mode : Mode
}

pub fn handle_args() -> Args {
    //let mut season_nb = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough args");
    }
    //let mut folder_path = args.last().unwrap();
    let mut folder_path = "".to_string();
    //let mut found_arg_season_nb = false;
    let mut mode = Mode { append_season: None, pattern_remover: None };
    let mut pos = 0;
    let mut arg_iter = args.iter();
    let mut arg = arg_iter.next().unwrap();
    while pos < args.len() {
        println!("arg : {}", arg);
        if arg == "--season-append"{
            arg = arg_iter.next().unwrap();
            pos = pos + 1;
            if !arg.parse::<i32>().is_ok() {
                panic!("season number argument is not a number");
            }
            let season_nb = arg.parse::<i32>().unwrap();
            //mode = Mode::AppendSeason(AppendSeasonArgs{ season_number: season_nb });
            mode.append_season = Some(AppendSeasonArgs { season_number: season_nb });
        } else if arg == "--pattern-remove" {
            arg = arg_iter.next().unwrap();
            pos = pos + 1;
            //mode = Mode::AppendSeason(AppendSeasonArgs{ season_number: season_nb });
            mode.pattern_remover = Some(PatternRemoverArgs { pattern: arg.clone() });
        } else {
            if arg.starts_with("--"){
                panic!("Unkown arg : {}", arg);
            }
            folder_path = arg.to_string(); 
            println!("found folder path : {}", folder_path);
            //panic!("Unkown arg : {}", arg);
        }
        println!("pos : {}, args.len : {}", pos, args.len());
        if pos+1 < args.len() {
            arg = arg_iter.next().unwrap();
        }
        pos = pos + 1;
    }
    /*for arg in args {
        if found_arg_season_nb && arg.parse::<i32>().is_ok() {
            //season_nb = arg.parse::<i32>().unwrap();
            let season_nb = arg.parse::<i32>().unwrap();
            mode = Mode::AppendSeason(AppendSeasonArgs{ season_number: season_nb });
        } else {
        if arg == "--season-append"{
            found_arg_season_nb = true;
        } else if (arg  == "--remove-pattern") {
        } else {
            found_arg_season_nb = false;
            folder_path = arg;
        }
        }
    }*/
    Args { folder_path: folder_path, mode : mode}
}