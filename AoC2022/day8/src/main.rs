use std::collections::HashMap;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>     {
    let data = fs::read_to_string("data.txt")?;

    // let split: Vec<&str> = data.split("\r\n").collect();

    // let mut map:HashMap<(i32, i32), (i32, i32, i32, i32, i32)> = HashMap::new();


    let rows:Vec<Vec<u32>> = data.split_terminator("\n")
                 .map(|col| col.chars().filter_map(|c|
                     c.to_digit(10)).collect()).collect();

    // Shits gonna be fking slow

    let mut visbile_counter = 0;
    
    //println!("Data is {:?}", rows);


    let mut t: u32 = 0;
    let mut m: u32 = 0;
    for (y, row) in rows.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            // HORIZONTAL
            if x == 0 || x == row.len() - 1 {
                t += 1;
                // View == 0, so we can
                continue;
            }
            // VERTICAL
            if y == 0 || y == rows.len() - 1 {
                t += 1;
                // View == 0, so we can
                continue;
            }
            // Loop each height: h
            let mut visible = false;
            // LEFT
            let mut left = 0;
            for i in (0..x).rev() {
                if row[i] < *h {
                    left += 1;
                    // Visible
                    if i == 0 && !visible {
                        visible = true;
                        t += 1;
                    }
                } else {
                    left += 1;
                    break;
                }
            }
            // RIGHT
            let mut right = 0;
            for i in x + 1..row.len() {
                if row[i] < *h {
                    right += 1;
                    // Visible
                    if i == row.len() - 1 && !visible {
                        visible = true;
                        t += 1;
                    }
                } else {
                    right += 1;
                    break;
                }
            }
            // UP
            let mut up = 0;
            for j in (0..y).rev() {
                if rows[j][x] < *h {
                    up += 1;
                    // Visible
                    if j == 0 && !visible {
                        visible = true;
                        t += 1;
                    }
                } else {
                    up += 1;
                    break;
                }
            }
            // DOWN
            let mut down = 0;
            for j in y + 1..rows.len() {
                if rows[j][x] < *h {
                    down += 1;
                    // Visible
                    if j == rows.len() - 1 && !visible {
                        visible = true;
                        t += 1;
                    }
                } else {
                    down += 1;
                    break;
                }
            }

            // Calculate view
            let v = right * left * up * down;
            if v > m {
                m = v;
            }
        }
    }

    println!("Part 1: {}", t);
    println!("Part 2: {}", m);

    Ok(())
}
