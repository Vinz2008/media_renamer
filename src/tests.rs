#[cfg(test)]
mod tests {
    #[test]
    fn one_number_test() {
        let filename = "series - 1 - [BDrip] - (2022)".to_string();
        
        let renamed_filename = crate::rename_file(&filename, 1);
        assert!(renamed_filename == "series - S01E01 - [BDrip] - (2022)")
    }
    #[test]
    fn two_number_test() {
        let filename = "series - 10 - [BDrip] - (2022)".to_string();
        
        let renamed_filename = crate::rename_file(&filename, 1);
        assert!(renamed_filename == "series - S01E10 - [BDrip] - (2022)")
    }
}