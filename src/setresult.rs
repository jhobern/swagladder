// Not doing draws because I don't need it at all.
// It wouldn't be hard to implement if you need it.

use utils;

// Also, why are the two most sensible names for this, result and match,
// both builtin Rust things?  Super annoying.

#[derive(RustcDecodable, RustcEncodable, Default)]
pub struct SetResult {
    time: String, // TODO Rust time object? serialization?
    winner: String, // TODO Player struct? any benefit?
    loser: String,
    winner_score: Option<u64>,
    loser_score: Option<u64>,
    winner_character: Option<String>, // TODO enum of characters?
    loser_character: Option<String>, // TODO multiple characters?
    notes: Option<String>
}

impl SetResult {
    pub fn new(winner: &String, loser: &String) -> SetResult {
        SetResult {
            time: utils::get_current_time(),
            winner: winner.clone(),
            loser: loser.clone(),
            ..Default::default()
        }
    }

    pub fn new_with_time(time: &String, winner: &String, loser: &String) -> SetResult {
        SetResult {
            time: time.clone(),
            winner: winner.clone(),
            loser: loser.clone(),
            ..Default::default()
        }
    }
}
