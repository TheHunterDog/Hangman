use std::io;
mod game;
fn main() {
  let mut word = String::new();

  println!("Enter the word the other player has to guess:");
  io::stdin()
  .read_line(&mut word)
  .expect("Error while reading the input");

    let mut game = game::build_game(word);
    while game.playing() {
        let mut gues = String::new();

        game.print_screen();
        println!("Enter a word to take a guess:");
        io::stdin()
            .read_line(&mut gues)
            .expect("Error while reading the input");
        game.guess(gues);
    }
}
