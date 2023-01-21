use std::fs;
use std::error::Error;


enum Gestures {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

const WIN:i32 = 6;
const LOSE:i32 = 0;
const DRAW:i32 = 3;


impl From<&&str> for Gestures {
    fn from(play: &&str) -> Self {
        match play {
            &"A" => { Gestures::Rock }
            &"B" => { Gestures::Paper }
            &"C" => { Gestures::Scissors }
            &"X" => { Gestures::Rock }
            &"Y" => { Gestures::Paper }
            &"Z" => { Gestures::Scissors }
            _ => {Gestures::Rock}
        }
    }
}


impl Gestures {
    fn versus(&self, opponent_play: &Gestures) -> i32 {
        let mut result = 0;

        let user_play_ordinal:i32 = self.ordinalise();
        let opponent_play_ordinal:i32 = opponent_play.ordinalise();

        let diff = user_play_ordinal - opponent_play_ordinal;
        result += user_play_ordinal;

        if diff == 1 || diff == -2 {
            result += WIN;
        } 
        else if diff == 0 {
            result += DRAW;
        }
        else {
            result += LOSE;
        }

        result
    }


    fn ordinalise(&self) -> i32 {
        match self {
            Gestures::Rock => 1,
            Gestures::Paper => 2,
            Gestures::Scissors => 3,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut total_score = 0;

    let mut score_v2 = 0;

    let data = fs::read_to_string("data.txt")?;

    let split: Vec<&str> = data.split("\r\n").collect();

    for round in split {
        let round_split:Vec<&str> = round.split(" ").collect();

        let opponent_play:Gestures = round_split.get(0).unwrap().into();

        let user_play:Gestures = round_split.get(1).unwrap().into();

        match round_split.get(1).unwrap() {
            &"X" => { score_v2 += (LOSE + wrap_around(opponent_play.ordinalise() - 1))}
            &"Y" => { score_v2 += (DRAW + opponent_play.ordinalise()) }
            &"Z" => { score_v2 += (WIN  + wrap_around(opponent_play.ordinalise() + 1))}
            _ => {}
        }

        total_score += user_play.versus(&opponent_play);
    }


    println!("Hello, world!");

    println!("The Score isss {}", total_score);

    println!("The correct score acc to elf: {}", score_v2);

    return Ok(());
}


fn wrap_around(num:i32) -> i32 {
    if num == 0 {
        return 3;
    }
    else if num == 4 {
        return 1;
    }
    num
}