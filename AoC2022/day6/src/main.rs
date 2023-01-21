use std::collections::{HashMap, VecDeque};
use std::fs;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data.txt")?;

    let mut char_record_map:HashMap<char, i32> = HashMap::new();

    let mut char_stack:VecDeque<char> = VecDeque::new();

    let mut non_distinct_counter = 0;


    for (index, character) in data.chars().enumerate() {

        char_stack.push_back(character);
        if index > 13 {
            let poped_char = char_stack.pop_front().unwrap();
            let counter = char_record_map.get_mut(&poped_char).unwrap();
            if *counter > 1 {
                // println!("Poped is {}", poped_char);
                // println!("Yessss");
                non_distinct_counter -= 1;
            }
            *counter -= 1;
        }

        match char_record_map.get_mut(&character) {
            Some(counter) => {
                if *counter != 0 {
                    non_distinct_counter += 1;
                }
                *counter += 1;
            },
            None => {
                char_record_map.insert(character, 1);
            },
        };

        if index >= 13 && non_distinct_counter == 0 {
            println!("I have found it! {}", index + 1);
            break;
        }
    }

    Ok(())
}
