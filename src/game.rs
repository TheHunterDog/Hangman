pub struct Game {
    lives: i8,
    screens: [String; 7],
    word: String,
    display: Vec<char>,
    playing: bool,
}

impl Game {
    pub fn playing(&self) -> bool {
        self.playing
    }
    pub fn print_screen(&self) {
        println!("The word has {} chars", self.word.chars().count());
        for a in self.display.iter() {
            print!(" {} ", a)
        }
        println!("{}", self.screens[(self.lives - 7).abs() as usize]);
    }
    pub fn guess(&mut self, gues: &str) {
        if gues.chars().count() == 1 {
          let mut found: bool = false;
          let gues_chars = gues.chars().next().unwrap();
            for (index,a) in self.word.chars().enumerate() {
                if gues_chars == a {
                  self.display[index] = a;
                  found = true;
                }
            }
            if !found {
              self.removelive();
            }
            else{
              self.check_win(); 
            }
        }else{
        if !self.word.eq(&gues.trim()) {
            self.removelive();
        } else {
            self.youwin();
        }
      }
    }
    fn check_win(&mut self){
      if !self.display.contains(&'_'){
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
  let mut display:Vec<char> = Vec::new();
  for _a in chosen_word.trim().chars() {
    display.push('_')
  }
    Game {
        playing: true,
        lives: 7,
        word: chosen_word.trim().to_string(),
        display: display,
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
