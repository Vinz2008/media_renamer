pub fn append_season(filename : &String, season_nb : i32) -> String {
    let mut new_filename = filename.clone();
    let mut pos = 0;
    let mut iter = filename.chars();
    let mut c = iter.next().unwrap();
    // TODO : add a last char variable to find if E is before the number so it will work even with big number that could be years
    // let mut last_char;
    while pos + 1 < filename.len() {
        if c.is_ascii_digit(){
            let mut number_pos = pos;
            // needed to fix strange bug where number_pos is not the pos of a number for whatever reason (see the test_for_strange_bug test)
            if !filename.as_bytes()[number_pos].is_ascii_digit(){
                number_pos = number_pos+1;
            }
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
            println!("new_filename[{}] : {}", number_pos, new_filename.as_bytes()[number_pos]);
            //number_pos = number_pos+1;
            new_filename.replace_range(number_pos..number_pos+number.len(), "");
            println!("new_filename after removing : {}", new_filename);
            new_filename.insert_str(number_pos, &number_replacement);
            println!("new_filename after replacing : {}", new_filename);
            if pos + 1 > filename.len() {
                break;
            }
            }
        } else {
            let temp = iter.next();
            if temp.is_none(){
                break;
            }
            c = temp.unwrap();
            pos = pos + 1;
        }
    }

    new_filename
}


// remove junk between season and episode number : will mostly be " - " which will be replaced by E
pub fn dash_remover(filename : &String) -> String {
    let mut new_filename = filename.clone();
    let mut pos = 0;
    let mut iter = filename.chars();
    let mut c = iter.next().unwrap();
    // TODO : add a last char variable to find if E is before the number so it will work even with big number that could be years
    // let mut last_char;
    while pos + 1 < filename.len() {
        if c == 'S'{
            c = iter.next().unwrap();
            pos = pos + 1;
            if c.is_ascii_digit() {
            while c.is_ascii_digit() && pos + 1 < filename.len(){
                c = iter.next().unwrap();
                pos = pos + 1;
            }
            let pos_start = pos;
            println!("pos_start : {}", pos_start);
            while !c.is_ascii_digit() && pos + 1 < filename.len(){
                c = iter.next().unwrap();
                pos = pos + 1;
            }
            let pos_end = pos;
            let len = pos_end - pos_start;
            let mut pos_removing : i32 = 0;
            while pos_removing < len as i32 {
                new_filename.remove(pos_start as usize);
                pos_removing = pos_removing+1;
                println!("new_filename in loop : {}", new_filename);
            }
            new_filename.insert(pos_start, 'E');
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