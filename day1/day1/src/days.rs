
pub mod day1 {
    use std::fs;
    pub fn day1(){
        let file_path = "input.txt";
        let mut max_list: Vec<u32> = Vec::new();
        let mut running: u32 = 0;
        println!("In file {}", file_path);
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        let split = contents.split('\n');
        for val in split {
            if val.eq(&String::from("")){
                max_list.push(running);
                running = 0;   
            }
            else {
                running += val.parse::<u32>().unwrap();
            }
        }
        max_list.sort_by(|a, b| b.cmp(a)); 
        println!("The top three are: {}", max_list[0] + max_list[1] + max_list[2]);
    }
}

pub mod day2{
    use std::fs;
    pub fn day2(){
        let file_path = "day21.txt";
        let mut match_score : u32 = 0;
        let mut match_outcome : u32 = 0;
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        let strat_list = contents.split_whitespace();
        for strat in strat_list {
            match strat {
                "A" => match_outcome += 0,
                "B" => match_outcome += 1,
                "C" => match_outcome += 2,
                "X" => {
                    match_outcome += 2;
                    match_score += (match_outcome % 3) + 1;
                    match_outcome = 0;
                }
                "Z" => {
                    match_outcome += 1;
                    match_score += (match_outcome % 3) + 1;
                    match_score += 6;
                    match_outcome = 0;
                }
                "Y" => {
                    match_score += match_outcome + 1;
                    match_score += 3;
                    match_outcome = 0;
                }
                _ => println!("Something went wrong!"),
            }
        }
        println!("{}", match_score);
    }
}

pub mod day3{
    use std::fs;
    use std::collections::HashSet;
    pub fn day3(){
        let file_path = "day3.txt";
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        let content_list = contents.split_whitespace();
        let mut score = 0;
        let mut first: HashSet<char> = HashSet::new();
        let mut second: HashSet<char> = HashSet::new();
        let mut third: HashSet<char> = HashSet::new();
        let mut index = 0;
        for contents in content_list {
            match index{
                0 => day3_set_creator(contents, &mut first),
                1 => day3_set_creator(contents, &mut second),
                2 => {
                    day3_set_creator(contents, &mut third);
                    for shared in first.intersection(&second){
                        if third.contains(shared) {
                            score += day3_score(shared);
                        }
                    }
                    first.clear();
                    second.clear();
                    third.clear();
                },
                _ => println!("error")
    
            }
            index += 1;
            index %= 3;
        }
        println!("{}", score);
    }
    
    fn day3_set_creator(input_string : &str,  set : &mut HashSet<char>  ){
        for c in input_string.chars(){
            set.insert(c);
        }
    }
    
    fn day3_score(c: &char) -> u32{
        let mut score = 0;
        let mut c1 : char = *c;
        if c.is_ascii_uppercase(){
            score +=26;
            for x in c.to_lowercase(){
                c1 = x;
            }
        }
        match c1 {
            'a' => score += 1,
            'b' => score += 2,
            'c' => score += 3,
            'd' => score += 4,
            'e' => score += 5,
            'f' => score += 6,
            'g' => score += 7,
            'h' => score += 8,
            'i' => score += 9,
            'j' => score += 10,
            'k' => score += 11,
            'l' => score += 12,
            'm' => score += 13,
            'n' => score += 14,
            'o' => score += 15,
            'p' => score += 16,
            'q' => score += 17,
            'r' => score += 18,
            's' => score += 19,
            't' => score += 20,
            'u' => score += 21,
            'v' => score += 22,
            'w' => score += 23,
            'x' => score += 24,
            'y' => score += 25,
            'z' => score += 26,
            _ => score += 0,
        }
        return score;
    }
}