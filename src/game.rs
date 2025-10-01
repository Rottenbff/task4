use crate::{fair_random, morty::Morty, stats::GameStats};
use std::io;

pub struct Game {
    num_boxes: u32,
    morty: Box<dyn Morty>,
    stats: GameStats,
}

impl Game {
    pub fn new(num_boxes: u32, morty: Box<dyn Morty>) -> Self {
        Self {
            num_boxes,
            morty,
            stats: GameStats::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.play_round();
            println!("\nMorty: D-do you wanna play another round (y/n)?");
            if !self.prompt_yes_no() {
                break;
            }
        }
        println!("Morty: Okay... uh, bye!");
        self.stats.display(self.num_boxes, self.morty.as_ref());
    }

    fn play_round(&mut self) {
        println!("\n--- NEW ROUND ---");
        println!("Morty: Oh geez, Rick, I'm gonna hide your portal gun in one of the {} boxes, okay?", self.num_boxes);

        let prize_protocol_prompt = &format!("Morty: Rick, enter your number (0 to {}) so you don’t whine later that I cheated, alright?", self.num_boxes - 1);
        let rick_value1 = self.prompt_for_u32(prize_protocol_prompt, self.num_boxes);
        let prize_protocol_result = fair_random::run_protocol(self.num_boxes, rick_value1);
        let prize_box = prize_protocol_result.final_value;

        let rick_choice = self.prompt_for_u32(
            &format!("Morty: Okay, okay, I hid the gun. What’s your guess (0 to {})?", self.num_boxes - 1),
            self.num_boxes,
        );

        let (opened_boxes, other_box, second_protocol_result) = self.morty.decide_action(
            self.num_boxes,
            prize_box,
            rick_choice,
        );

        println!("Morty: I'm opening boxes {:?}. Your box is {} and the other one is {}.", opened_boxes, rick_choice, other_box);

        println!("Morty: You can switch your box to {} (enter 0), or, you know, stick with it (enter 1).", other_box);
        let stayed = self.prompt_for_u32("Your choice (0 or 1):", 2) == 1;
        let final_choice = if stayed { rick_choice } else { other_box };

        println!("\nMorty: The portal gun is in the box {}.", prize_box);
        let won = final_choice == prize_box;
        if won {
            println!("Morty: Aww geez, you won, Rick.");
        } else {
            println!("Morty: Aww man, you lost, Rick. Now we gotta go on one of *my* adventures!");
        }

        println!("\n--- Protocol Reveal ---");
        println!("Morty: Aww man, my 1st random value was {}.", prize_protocol_result.morty_value);
        println!("Morty: KEY1={}", hex::encode_upper(prize_protocol_result.key));
        println!("Morty: So the 1st fair number is ({} + {}) % {} = {}.", prize_protocol_result.rick_value, prize_protocol_result.morty_value, self.num_boxes, prize_box);

        if let Some(result) = second_protocol_result {
            let max = self.num_boxes - 1;
            println!("Morty: Aww man, my 2nd random value is {}.", result.morty_value);
            println!("Morty: KEY2={}", hex::encode_upper(result.key));
            println!("Morty: So the 2nd fair number is ({} + {}) % {} = {}.", result.rick_value, result.morty_value, max, result.final_value);
        }

        self.stats.record_round(stayed, won);
    }

    fn prompt_for_u32(&self, prompt: &str, max_exclusive: u32) -> u32 {
        loop {
            println!("{}", prompt);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<u32>() {
                Ok(num) if num < max_exclusive => return num,
                _ => println!("Morty: That's not a valid number, Rick! It must be between 0 and {}.", max_exclusive - 1),
            }
        }
    }

    fn prompt_yes_no(&self) -> bool {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_lowercase().starts_with('y')
    }
}
