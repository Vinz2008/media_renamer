#[cfg(test)]
mod tests {
    #[test]
    fn season_append_one_number() {
        let filename = "series - 1 - [BDrip] - (2022)".to_string();
        
        let renamed_filename = crate::rename_file(&filename, 1);
        assert!(renamed_filename == "series - S01E01 - [BDrip] - (2022)");
    }
    #[test]
    fn season_append_two_number(){
        let filename = "series - 10 - [BDrip] - (2022)".to_string();
        
        let renamed_filename = crate::rename_file(&filename, 1);
        assert!(renamed_filename == "series - S01E10 - [BDrip] - (2022)");
    }
    #[test]
    fn pattern_remove(){
        let filename = "series - 10 - [BDrip] - (2022)".to_string();
        let pattern = "BDrip".to_string();
        let renamed_filename = crate::remove_pattern(&filename, &pattern);
        assert!(renamed_filename == "series - 10 - [] - (2022)");
    }
    #[test]
    fn pattern_remove2(){
        let filename = "series.x264.HP.AC3-2.0.(2022)".to_string();
        let pattern = "x264.HP.AC3-2.0.".to_string();
        let renamed_filename = crate::remove_pattern(&filename, &pattern);
        assert!(renamed_filename == "series.(2022)");
    }

}