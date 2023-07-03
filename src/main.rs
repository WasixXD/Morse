

use ncurses::*;
use std::char;
use std::collections::HashMap;




fn prompt() -> String {
    
    let mut letter = String::new();
    loop {
        let ch = getch();

        if ch == 'a' as i32 {
            addstr(".");
            letter.push_str(".");
        } else if ch == 's' as i32 {
            addstr("-");
            letter.push_str("-");
        } else if ch == ' ' as i32 {
            break;
        } else {
            return " ".to_string();
        }
    }
    
    letter

 
}



fn main() {
    let alphabet = HashMap::from([
        (".-".to_string(), "A"),
        ("-...".to_string(), "B"),
        ("-.-.".to_string(), "C"),
        ("-..".to_string(), "D"),
        (".".to_string(), "E"),
        ("..-.".to_string(), "F"),
        ("--.".to_string(), "G"),
        ("....".to_string(), "H"),
        ("..".to_string(), "I"),
        (".---".to_string(), "J"),
        ("-.-".to_string(), "K"),
        (".-..".to_string(), "L"),
        ("--".to_string(), "M"),
        ("-.".to_string(), "N"), 
        ("---".to_string(), "O"),
        (".--.".to_string(), "P"),
        ("--.-".to_string(), "Q"),
        (".-.".to_string(), "R"),
        ("...".to_string(), "S"),
        ("-".to_string(), "T"),
        ("..-".to_string(), "U"),
        ("...-".to_string(), "V"),
        (".--".to_string(), "W"),
        ("-..-".to_string(), "X"),
        ("-.--".to_string(), "Y"),
        ("--..".to_string(), "Z"),
    ]);
    initscr();
    raw();
    noecho();
   
    let mut text = String::new();
    loop {

       
        let q = prompt();

        let letter = alphabet.get(&q);
        if q == " " {
            break;
        }
        clear();
        match letter {
            Some(valid) => text.push_str(valid),
            None => break
        };
               
        addstr(&text);
        refresh();
    }
       
    getch();
    endwin();
  

    
}
