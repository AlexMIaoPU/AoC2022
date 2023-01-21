use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut largest_calo = 0;

    let mut calo_collections:Vec<u32> = Vec::new();

    let data = fs::read_to_string("src/data.txt")?;

    let split: Vec<&str> = data.split("\r\n").collect();

    //println!("{:?}", split);

    let mut this_elf_calo = 0;

    for single_elf_data in split {

        if single_elf_data == "" {
            calo_collections.push(this_elf_calo);
            this_elf_calo = 0;
        }
        else {
            let int_unit_food_calo:u32 = single_elf_data.parse().unwrap();
        
            this_elf_calo += int_unit_food_calo;
        }

        if this_elf_calo > largest_calo {
            largest_calo = this_elf_calo;
        }

    }

    println!("Largest is {}", largest_calo);

    calo_collections.sort();
    calo_collections.reverse();

    println!("The first three has the sum of {}", calo_collections[0] + calo_collections[1] + calo_collections[2]);

    return Ok(());
}
