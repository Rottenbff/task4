use crate::fair_random;
use std::io;

fn prompt_for_u32(prompt: &str, max_exclusive: u32) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(num) if num < max_exclusive => return num,
            _ => println!(
                "Morty: Invalid input, Rick! It must be a number between 0 and {}.",
                max_exclusive - 1
            ),
        }
    }
}

pub trait Morty {
    fn decide_action(
        &self,
        total_boxes: u32,
        prize_box: u32,
        rick_choice: u32,
    ) -> (Vec<u32>, u32, Option<fair_random::FairRandomResult>);

    fn calculate_win_prob_switch(&self, n: u32) -> f64 {
        (n as f64 - 1.0) / n as f64
    }
    fn calculate_win_prob_stay(&self, n: u32) -> f64 {
        1.0 / n as f64
    }
}

pub struct ClassicMorty;
impl Morty for ClassicMorty {
    fn decide_action(
        &self,
        total_boxes: u32,
        prize_box: u32,
        rick_choice: u32,
    ) -> (Vec<u32>, u32, Option<fair_random::FairRandomResult>) {
        let other_box_to_keep: u32;
        let second_protocol_result: Option<fair_random::FairRandomResult>;

        if rick_choice == prize_box {
            println!("\nMorty: Uh oh, Rick. You were right. Let's, uh, generate another value to see which other box I'll keep.");

            let available_choices: Vec<u32> =
                (0..total_boxes).filter(|&i| i != rick_choice).collect();
            let prompt = &format!(
                "Morty: Rick, enter your number for the 2nd HMAC (0 to {})",
                available_choices.len() - 1
            );
            let rick_value2 = prompt_for_u32(prompt, available_choices.len() as u32);

            let protocol_result =
                fair_random::run_protocol(available_choices.len() as u32, rick_value2);
            other_box_to_keep = available_choices[protocol_result.final_value as usize];
            second_protocol_result = Some(protocol_result);
        } else {
            println!("\nMorty: Alright, Rick. Let's see what happens here (winks).");
            other_box_to_keep = prize_box;
            second_protocol_result = None;
        }

        let opened_boxes = (0..total_boxes)
            .filter(|&i| i != rick_choice && i != other_box_to_keep)
            .collect();

        (opened_boxes, other_box_to_keep, second_protocol_result)
    }
}

pub struct LazyMorty;
impl Morty for LazyMorty {
    fn decide_action(
        &self,
        total_boxes: u32,
        prize_box: u32,
        rick_choice: u32,
    ) -> (Vec<u32>, u32, Option<fair_random::FairRandomResult>) {
        let mut opened_boxes = Vec::new();
        let num_to_open = total_boxes - 2;

        for i in 0..total_boxes {
            if opened_boxes.len() as u32 == num_to_open {
                break;
            }
            if i != rick_choice && i != prize_box {
                opened_boxes.push(i);
            }
        }

        let other_box_to_keep = (0..total_boxes)
            .find(|&i| i != rick_choice && !opened_boxes.contains(&i))
            .expect("Logic error: Should always be one box left to keep");

        (opened_boxes, other_box_to_keep, None)
    }
}
