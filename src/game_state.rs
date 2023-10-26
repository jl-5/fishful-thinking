pub struct GameState{
    pub chars_typed: u32,
    pub score: usize,
    pub score_changing: bool,
    pub is_currently_casted: bool,
    // 0 = title, 1 = instructions, 2 = play, 3 = end,
    pub game_screen: usize,
}

impl GameState {
    pub fn init_game_state() -> GameState {
        // any necessary functions
        GameState {
            chars_typed : 0,
            score : 0,
            score_changing : false,
            is_currently_casted: false,
            game_screen: 0,
        }
    }
}