pub fn rename_file(filename : &String, season_nb : i32) -> String {
    let mut new_filename = filename.clone();
    let mut pos = 0;
    let mut iter = filename.chars();
    let mut c = iter.next().unwrap();
    // TODO : add a last char variable to find if E is before the number so it will work even with big number that could be years
    // let mut last_char;
    while pos + 1 < filename.len() {
        if c.is_ascii_digit(){
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

pub fn remove_pattern(filename : &String, pattern : &String) -> String {
    filename.replace(pattern, "")
}