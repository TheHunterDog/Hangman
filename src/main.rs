
mod game;
mod menu;
fn main() {
  let mut menu = menu::construct_menu(vec!['s','q'],String::from("s = start q = quit"));
  while menu.play_menu() {
    let mut word = String::new();

    println!("Enter the word the other player has to guess:");
    std::io::stdin()
    .read_line(&mut word)
    .expect("Error while reading the input");
    print!("\x1B[2J");
      let mut game = game::build_game(word);
      while game.playing() {
          let mut gues = String::new();
  
          game.print_screen();
          println!("Enter a word to take a guess:");
          std::io::stdin()
              .read_line(&mut gues)
              .expect("Error while reading the input");
          game.guess(gues.trim());
      }
  }




}
