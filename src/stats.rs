use crate::morty::Morty;
use comfy_table::{Cell, Table};

#[derive(Default)]
pub struct GameStats {
    rounds_stayed: u32,
    wins_stayed: u32,
    rounds_switched: u32,
    wins_switched: u32,
}

impl GameStats {
    pub fn record_round(&mut self, stayed: bool, won: bool) {
        if stayed {
            self.rounds_stayed += 1;
            if won {
                self.wins_stayed += 1;
            }
        } else {
            self.rounds_switched += 1;
            if won {
                self.wins_switched += 1;
            }
        }
    }

    pub fn display(&self, total_boxes: u32, morty: &dyn Morty) {
        let mut table = Table::new();
        table.set_header(vec!["Game results", "Rick switched", "Rick stayed"]);

        let p_estimate_switch = if self.rounds_switched > 0 {
            format!("{:.3}", self.wins_switched as f64 / self.rounds_switched as f64)
        } else {
            "?".to_string()
        };

        let p_estimate_stay = if self.rounds_stayed > 0 {
            format!("{:.3}", self.wins_stayed as f64 / self.rounds_stayed as f64)
        } else {
            "?".to_string()
        };

        table.add_row(vec![
            Cell::new("Rounds"),
            Cell::new(self.rounds_switched),
            Cell::new(self.rounds_stayed),
        ]);
        table.add_row(vec![
            Cell::new("Wins"),
            Cell::new(self.wins_switched),
            Cell::new(self.wins_stayed),
        ]);
        table.add_row(vec![
            Cell::new("P (estimate)"),
            Cell::new(p_estimate_switch),
            Cell::new(p_estimate_stay),
        ]);
        table.add_row(vec![
            Cell::new("P (exact)"),
            Cell::new(format!("{:.3}", morty.calculate_win_prob_switch(total_boxes))),
            Cell::new(format!("{:.3}", morty.calculate_win_prob_stay(total_boxes))),
        ]);

        println!("\n                  GAME STATS");
        println!("{}", table);
    }
}
