pub fn print_usage(){
    println!("Usage : media-renamer [OPTIONS] FOLDER");
    print!("\n");
    println!("Options:");
    println!("  --help                          Display this message");
    println!("  --append-season SEASON_NUMBER   Append a season number to filenames with only an episode number");
    println!("  --remove-pattern PATTERN        Remove a pattern from filenames");
    println!("  --remove-dash                   Remove the dash (and everything else) between the season number and the episode number");
}