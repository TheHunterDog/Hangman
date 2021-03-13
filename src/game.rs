pub struct Game {
    lives: i8,
    screens: [String; 7],
    word: String,
    playing: bool,
}

impl Game {
    pub fn playing(&self) -> bool {
        self.playing
    }
    pub fn print_screen(&self) {
        println!("The word has {} chars", self.word.chars().count());
        println!("{}", self.screens[(self.lives - 7).abs() as usize]);
    }
    pub fn guess(&mut self, gues: String) {
        if !self.word.eq(&gues.trim()) {
            self.removelive();
        } else {
            self.youwin();
        }
    }
    fn removelive(&mut self) {
        if self.lives == 0 {
            self.playing = false;
        } else {
            self.lives -= 1;
        }
    }
    fn youwin(&mut self) {
        println!("You win EPIC");
        self.playing = false;
    }
}
pub fn build_game(chosen_word: String) -> Game {
    Game {
        playing: true,
        lives: 7,
        word: chosen_word.trim().to_string(),
        screens: [
            String::from(
                "
+---+
    |   |
        |
        |
        |
        |
  =========",
            ),
            String::from(
                "
    +---+
    |   |
    O   |
        |
        |
        |
  =========",
            ),
            String::from(
                "
    +---+
    |   |
    O   |
    |   |
        |
        |
  =========",
            ),
            String::from(
                "
    +---+
    |   |
    O   |
   /|   |
        |
        |
  =========",
            ),
            String::from(
                "
    +---+
    |   |
    O   |
   /|\"  |
        |
        |
  =========",
            ),
            String::from(
                "
    +---+
    |   |
    O   |
   /|\"  |
   /    |
        |
  =========",
            ),
            String::from(
                "
    +---+
    |   |
    O   |
   /|\"  |
   / \"  |
        |
  =========",
            ),
        ],
    }
}
