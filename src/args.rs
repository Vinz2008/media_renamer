use std::env;

pub struct AppendSeasonArgs {
    pub season_number : i32
}

pub struct PatternRemoverArgs {
    pub pattern : String
}

pub struct DashRemoverArgs;

// maybe use a vec of modes instead, but it need to be sorted first
pub struct Mode {
    pub append_season : Option<AppendSeasonArgs>,
    pub pattern_remover : Option<PatternRemoverArgs>,
    pub dash_remover : Option<DashRemoverArgs>,
}

pub struct Args {
    pub folder_path : String,
    pub mode : Mode,
    pub is_help_mode : bool
}

pub fn handle_args() -> Args {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Args { folder_path: "".to_string(), mode: Mode {append_season: None, pattern_remover: None, dash_remover: None}, is_help_mode: true};
    }
    let mut folder_path = "".to_string();
    let mut mode = Mode { append_season: None, pattern_remover: None, dash_remover: None };
    let mut pos = 0;
    let mut arg_iter = args.iter();
    let mut arg = arg_iter.next().unwrap();
    let mut is_help_mode = false;
    while pos < args.len() {
        //println!("arg : {}", arg);
        if arg == "--append-season"{
            arg = arg_iter.next().unwrap();
            pos = pos + 1;
            if !arg.parse::<i32>().is_ok() {
                panic!("season number argument is not a number");
            }
            let season_nb = arg.parse::<i32>().unwrap();
            mode.append_season = Some(AppendSeasonArgs { season_number: season_nb });
        } else if arg == "--remove-pattern" {
            arg = arg_iter.next().unwrap();
            pos = pos + 1;
            mode.pattern_remover = Some(PatternRemoverArgs { pattern: arg.clone() });
        } else if arg == "--remove-dash" {
            mode.dash_remover = Some(DashRemoverArgs);
        } else if arg == "--help" {
            is_help_mode = true;            
        } else {
            if arg.starts_with("--"){
                panic!("Unknown arg : {}", arg);
            }
            folder_path = arg.to_string(); 
            //println!("found folder path : {}", folder_path);
        }
        if pos+1 < args.len() {
            arg = arg_iter.next().unwrap();
        }
        pos = pos + 1;
    }
    Args { folder_path: folder_path, mode : mode, is_help_mode: is_help_mode}
}