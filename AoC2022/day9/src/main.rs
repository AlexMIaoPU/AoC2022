use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let data = fs::read_to_string("data.txt")?;

    let split: Vec<&str> = data.split("\r\n").collect();

    let mut commands: Vec<&str> = Vec::new();

    for line in split {
        let line_split: Vec<&str> = line.split(" ").collect();
        let num: i32 = line_split.get(1).unwrap().parse()?;

        for _i in 0..num {
            commands.push(line_split[0]);
        }
    }

    println!("Commands are {}",commands.len());


    // (x, y)

    let mut head = (0, 0);
    let mut old_head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = vec![(0, 0)];

    let mut pt_2_rope = vec![(0, 0); 10];
    let mut pt_2_visited = vec![(0, 0)];

    println!("pt 2 rope {:?}", pt_2_rope);

    for c in commands {
        old_head.1 = head.1;
        old_head.0 = head.0;
        match c {
            "U" => {
                head.1 += 1;
            },
            "D" => {
                head.1 -= 1;
            },
            "L" => {
                head.0 -= 1;
            },
            "R" => {
                head.0 += 1;
            },
            _ => {

            }
        }

        if ((head.0 - tail.0) as i32).abs() > 1 || ((head.1 - tail.1) as i32).abs() > 1 {
            tail.0 = old_head.0;
            tail.1 = old_head.1;
            if !visited.contains(&tail) {
                visited.push(tail);
            }
        }


        let mut prev_knot_old = (0, 0);
        let mut prev_knot_curr = (0, 0);
        for (index, knot) in pt_2_rope.iter_mut().enumerate() {
            let old = (knot.0, knot.1);
            if index == 0 {
                knot.0 = head.0;
                knot.1 = head.1;
                //println!("Index 0 at {:?}", knot);
            } else if index == 1 {
                if ((prev_knot_curr.0 - knot.0) as i32).abs() > 1 || ((prev_knot_curr.1 - knot.1) as i32).abs() > 1 {
                    knot.0 = old_head.0;
                    knot.1 = old_head.1;
                }
            } else {
                if ((prev_knot_curr.0 - knot.0) as i32).abs() > 1 || ((prev_knot_curr.1 - knot.1) as i32).abs() > 1 {

                    if (prev_knot_curr.0 == knot.0) || (prev_knot_curr.1 == knot.1) {
                        knot.0 += (prev_knot_curr.0 - knot.0)/2;
                        knot.1 += (prev_knot_curr.1 - knot.1)/2;
                    } else {
                        let diff_x = prev_knot_curr.0 - knot.0;
                        let diff_y = prev_knot_curr.1 - knot.1;
                        if diff_x > 0 {
                            knot.0 += 1;
                        } else {
                            knot.0 -= 1;
                        }

                        if diff_y > 0 {
                            knot.1 += 1;
                        } else {
                            knot.1 -= 1;
                        }
                    }

                    if index == 9 && !pt_2_visited.contains(knot) {
                        pt_2_visited.push(*knot);
                        //println!("999");
                    }
                }
            } 

            prev_knot_old = old;
            prev_knot_curr = (knot.0, knot.1);
        }
    }

    println!("Pt 1 {:?}", visited.len());
    println!("Pt 2 {:?}", pt_2_visited.len());
    Ok(())
}
