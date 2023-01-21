use std::collections::VecDeque;
use std::fs;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data.txt")?;

    let split: Vec<&str> = data.split("\r\n").collect();

    println!("lines count {}", split.len());

    let mut stacks:Vec<VecDeque<&str>> = Vec::new();

    for _i in 0..9 {
        stacks.push(VecDeque::new());
    }

    let mut stack_set_up_complete = false;

    // macro_rules! move_crate {
    //     (move $num:literal from $from:literal to $to:literal) => {
    //         stacks.get_mut(1).unwrap().push("ADFWDW");
    //     };
    //     () => {
    //         // do nothing
    //     }
    // }

    for line in split {

        if line.get(1..2).eq(&Some("1")) {
            stack_set_up_complete = true;
            continue;
        }

        if !stack_set_up_complete {
            let mut stack_index = 1;
            loop {
                let line_index = (stack_index - 1)*4 + 1;
                match line.get(line_index..line_index + 1) {
                    Some(crate_alpha) => {
                        if crate_alpha != " " {
                            let stack = stacks.get_mut(stack_index - 1).unwrap();
                            stack.push_front(crate_alpha);
                        }
                        stack_index += 1;
                    }
                    None => {
                        break;
                    }
                }
            }
        } else {
            line.trim_end();
            let line_split: Vec<&str> = line.split(" ").collect();
            if line_split.len() <= 1 { continue; }

            //println!("Line looks like {:?}", line_split);


            let num_moved: u32 = line_split.get(1).unwrap().parse().unwrap();
            let from: usize = line_split.get(3).unwrap().parse().unwrap();
            let to: usize = line_split.get(5).unwrap().parse().unwrap();

            let mut part_two_moved_stack:VecDeque<&str> = VecDeque::new();

            for _i in 0..num_moved {
                let moved_crate = stacks.get_mut(from - 1).unwrap().pop_back().unwrap();
                //stacks.get_mut(to - 1).unwrap().push_back(moved_crate);
                part_two_moved_stack.push_front(moved_crate);
            }

            stacks.get_mut(to - 1).unwrap().append(&mut part_two_moved_stack);
        }
    }

    println!("Stack looks like {:?}", stacks);

    Ok(())
}
