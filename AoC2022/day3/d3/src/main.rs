use std::collections::HashMap;
use std::fs;
use std::error::Error;

const GROUP_SIZE: i32 = 3;

fn main() -> Result<(), Box<dyn Error>> {


    let data = fs::read_to_string("data.txt")?;

    let split: Vec<&str> = data.split("\r\n").collect();

    let mut total_weight = 0;

    let mut total_badger_weight = 0;
    let mut group_member_counter = 0;
    let mut group_map:HashMap<char, i32> = HashMap::new();
    
    for line in split {

        group_member_counter += 1;

        let len = line.len();

        let first_half = &line[..len/2];
        let second_half = &line[len/2..len];

        let mut map:HashMap<char, i32> = HashMap::new();

        let mut dup_map:HashMap<char, i32> = HashMap::new();


        for char in first_half.chars() {
            map.insert(char, 1);
            if group_map.contains_key(&char) && *group_map.get(&char).unwrap() == (group_member_counter - 1) {
                println!("Char is {}, Current count {}, nth group member is {}", &char, group_map.get(&char).unwrap(), &group_member_counter);
                *group_map.get_mut(&char).unwrap() += 1;
            }
            if group_member_counter == 1 {
                group_map.insert(char, 1);
            }
        }

        for char in second_half.chars() {
            if map.get(&char).is_some() {
                dup_map.insert(char, 1);
            }
            if group_map.contains_key(&char) && *group_map.get(&char).unwrap() == (group_member_counter - 1) {
                println!("Char is {}, Current count {}, nth group member is {}", &char, group_map.get(&char).unwrap(), &group_member_counter);
                *group_map.get_mut(&char).unwrap() += 1;
            }
            if group_member_counter == 1 {
                group_map.insert(char, 1);
            }
        }

        for (k, v) in dup_map {
            if k.is_ascii_lowercase() {
                total_weight += ((k as i8 - 'a' as i8) as i32 + 1);
            }
            else {
                total_weight += ((k as i8 - 'A' as i8) as i32 + 27);
            }
        }

        if group_member_counter == GROUP_SIZE {
            println!("A Group");
            for (k, v) in group_map {
                if v == GROUP_SIZE {
                    println!("Badge is {}", &k);
                    if k.is_ascii_lowercase() {
                        total_badger_weight += ((k as i8 - 'a' as i8) as i32 + 1);
                    }
                    else {
                        total_badger_weight += ((k as i8 - 'A' as i8) as i32 + 27);
                    }
                }
            }
            group_map = HashMap::new();
            group_member_counter = 0;
        }
    }

    println!("Hello, world!");
    
    println!("The total Weight of the sack is {}", total_weight);

    println!("Badge {}", total_badger_weight);

    Ok(())
}
