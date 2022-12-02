use std::fs;

fn main() {
    let file_path = "input.txt";
    let mut max: u32 = 0;
    let mut running: u32 = 0;
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let split = contents.split('\n');
    for val in split {
        if val.eq(&String::from("")){
            if max < running {
                max = running;
                println!("new Max: {}", running);
            }
            running = 0;   
        }
        else {
            running += val.parse::<u32>().unwrap();
        }
    }
}