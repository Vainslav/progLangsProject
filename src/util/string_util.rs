pub fn remove_prefix_and_update_lines(text: String, prefix_number: usize, lines_length: usize, lines_offset: usize, lines_number: usize) -> String{
    if text.chars().last().unwrap() == '\n'{
        text.split_terminator("\n").skip(lines_offset).take(lines_number).map(|i| {
            let new_line: String = i.chars().skip(prefix_number).collect();
            let line: String = new_line.chars().take(lines_length).collect();
            line
        }).collect::<Vec<String>>().join("\n") + "\n"
    }else{
        text.split_terminator("\n").skip(lines_offset).take(lines_number).map(|i| {
            let new_line: String = i.chars().skip(prefix_number).collect();
            let line: String = new_line.chars().take(lines_length).collect();
            line
        }).collect::<Vec<String>>().join("\n")
    }
}

#[cfg(test)]
mod remove_prefix_and_update_lines_tests{
    use crate::util::string_util::remove_prefix_and_update_lines;

    #[test]
    fn test_1(){
        assert_eq!("12312", remove_prefix_and_update_lines("123123".to_string(), 0, 5, 0, 10))
    }

    #[test]
    fn test_2(){
        assert_eq!("123123", remove_prefix_and_update_lines("123123".to_string(), 0, 6, 0, 10))
    }
    
    #[test]
    fn test_3(){
        assert_eq!("12\n12", remove_prefix_and_update_lines("123\n123".to_string(), 0, 2, 0, 10))
    }
    
    #[test]
    fn test_4(){
        assert_eq!("23\n23", remove_prefix_and_update_lines("123\n123".to_string(), 1, 2, 0, 10))
    }
    
    #[test]
    fn test_5(){
        assert_eq!("23\n\n56", remove_prefix_and_update_lines("123\n\n456".to_string(), 1, 2, 0, 10))
    }
}