use std::env;
use std::fs;
use std::vec;

// TODO : add remover of pattern
// example : filename is "series - 01 - ac3"
// media_renamer --remover ac3 .
// will output the filename without ac3 : "series - 01 - "

fn rename_file(filename : &String, season_nb : i32) -> String {
    let mut new_filename = filename.clone();
    let mut pos = 0;
    let mut iter = filename.chars();
    let mut c = iter.next().unwrap();
    // TODO : add a last char variable to find if E is before the number so it will work even with big number that could be years
    // let mut last_char;
    while pos + 1 < filename.len() {
        //println!("{}", c.unwrap());

        if c.is_ascii_digit(){
            //println!("{}", c);
            let number_pos = pos;
            let mut number : String = "".to_string();
            while c.is_ascii_digit() && pos + 1 < filename.len(){
                number.push(c);
                c = iter.next().unwrap();
                pos = pos + 1;
            }
            println!("digit : {}", number);
            println!("digit_number : {}", number.len());
            if number.len() < 3 { // maybe <=
            let number_replacement = if number.len() == 1 {
                if season_nb < 10{
                    format!("S0{}E0{}", season_nb, number)
                } else {
                    format!("S{}E0{}", season_nb, number)
                }
            } else {
                if season_nb < 10{
                    format!("S0{}E{}", season_nb, number)
                } else {
                    format!("S{}E{}", season_nb, number)
                }
            };
            println!("number_replacement : {}", number_replacement);
            let mut pos_removing_number : i32 = 0;
            while pos_removing_number < (number.len()) as i32 {
                new_filename.remove(number_pos as usize);
                pos_removing_number = pos_removing_number+1;
                println!("new_filename in loop : {}", new_filename);
            }
            let mut pos_c_number = 0;
            for c_number_replacement in number_replacement.chars() { 
                println!("{}", c_number_replacement);
                new_filename.insert(number_pos+pos_c_number, c_number_replacement);
                pos_c_number = pos_c_number + 1;
            }
            println!("new_filename : {}", new_filename);
            if pos + 1 > filename.len() {
                break;
            }
            }
        } else {
            c = iter.next().unwrap();
            pos = pos + 1;
        }
    }

    new_filename
}

fn main() {
    let mut season_nb = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough args");
    }
    //let mut folder_path = args.last().unwrap();
    let mut folder_path = "".to_string();
    let mut found_arg_season_nb = false;
    for arg in args {
        if found_arg_season_nb && arg.parse::<i32>().is_ok() {
            season_nb = arg.parse::<i32>().unwrap();
        } else {
        if arg == "--season-nb"{
            found_arg_season_nb = true;
        } else {
            found_arg_season_nb = false;
            folder_path = arg;
        }
        }
    }
    println!("{}", folder_path);
    let paths = fs::read_dir(folder_path).unwrap();
    let mut files_and_renamed_files : Vec<(String, String)> = vec![];
    for path in paths {
        let file_path = path.unwrap().path().into_os_string().into_string().unwrap();
        println!("Name: {}", file_path);
        let renamed_file = rename_file(&file_path, season_nb);
        println!("new name : {}", renamed_file);
        files_and_renamed_files.push((file_path, renamed_file));
        //rename(file_path, renamed_file).unwrap();
    }

    for pairs in files_and_renamed_files {
        fs::rename(pairs.0, pairs.1).unwrap();
    } 
}

#[cfg(test)]
mod tests;