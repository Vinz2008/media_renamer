#[cfg(test)]
mod tests {
    #[test]
    fn season_append_one_number() {
        let filename = "series - 1 - [BDrip] - (2022)".to_string();
        
        let renamed_filename = crate::append_season(&filename, 1, None);
        assert!(renamed_filename == "series - S01E01 - [BDrip] - (2022)");
    }
    #[test]
    fn season_append_two_number(){
        let filename = "series - 10 - [BDrip] - (2022)".to_string();
        
        let renamed_filename = crate::append_season(&filename, 1, None);
        assert!(renamed_filename == "series - S01E10 - [BDrip] - (2022)");
    }
    #[test]
    fn season_append_two_number2(){
        let filename = "series - 01 - [BDrip] - (2022)".to_string();
        
        let renamed_filename = crate::append_season(&filename, 1, None);
        assert!(renamed_filename == "series - S01E01 - [BDrip] - (2022)");
    }
    #[test]
    fn season_append_episode_more_than_three_nb(){
        let filename = "Oshin (297).mp4".to_string();
        let renamed_filename = crate::append_season(&filename, 1, Some(4));
        println!("renamed_filename : {}", renamed_filename);
        assert!(renamed_filename == "Oshin (S01E297).mp4");
    }
    #[test]
    fn dash_remover(){
        let filename = "series S01 - 10 - [BDrip] - (2022)".to_string();
        let renamed_filename = crate::dash_remover(&filename);
        assert!(renamed_filename == "series S01E10 - [BDrip] - (2022)");
    }
    #[test]
    fn dash_remover2(){
        let filename = "Gangsta  S01 - 01 VOSTFR [1080p][X265][10BITS][SR-71].mkv".to_string();
        let renamed_filename = crate::dash_remover(&filename);
        assert!(renamed_filename == "Gangsta  S01E01 VOSTFR [1080p][X265][10BITS][SR-71].mkv");
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
    #[test]
    fn test_for_strange_bug(){
        let filename = "Versailles no Bara (Lady Ocscar) 1979 Intégrale MULTI BluRay (1080p x265 10bit FLAC 2.0) (JG666) - 1.mkv".to_string();
        let pattern = " 10bit FLAC 2.0".to_string();
        let renamed_filename = crate::remove_pattern(&filename, &pattern);
        let renamed_filename = crate::append_season(&renamed_filename, 1, None);
        assert!(renamed_filename == "Versailles no Bara (Lady Ocscar) 1979 Intégrale MULTI BluRay (1080p x265) (JG666) - S01E01.mkv");
    }
}