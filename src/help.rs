pub fn print_usage(){
    println!("Usage : media-renamer [OPTIONS] FOLDER");
    print!("\n");
    println!("Options:");
    println!("  --help               Display this message");
    println!("  --season-append      Append a season number to filenames with only an episode number");
    println!("  --pattern-remove     Remove a pattern from filenames");
    println!("  --dash-remove        Remove the dash (and everything else) between the season number and the episode number");
}