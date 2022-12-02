use std::fs;

fn main() {
   day1();
}

fn day1(){
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