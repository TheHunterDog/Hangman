
use std::io;

pub struct Menu {
    menu_screen: String,
    exit: bool,
}

impl Menu {
  pub fn play_menu(&mut self)->bool{
    self.menu_screen();
    self.menu_choice();

    if self.exit != true {
      true
    }
    else{
      std::process::exit(1);
    }
  }
  fn menu_screen(&self) {
        println!("{}", self.menu_screen)
    }

    fn menu_choice(&mut self) {
        let mut menu_ans: String = String::new();
        while menu_ans.trim().len() == 0 {
            io::stdin()
                .read_line(&mut menu_ans)
                .expect("Error while reading the input");
        }
        let ans_c = menu_ans.chars().next().unwrap();

        match ans_c {
            // 's' => ,
            'q' => self.exit = true,
            _ => self.exit = false,
        }
    }
}
pub fn construct_menu(menu_options: Vec<char>, menu_screen: String) -> Menu {
    Menu {
        menu_screen: menu_screen,
        exit: false,
    }
}
