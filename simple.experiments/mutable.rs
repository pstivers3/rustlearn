// does not compile

use std::io;

fn main() {
   
    let mut guess = "boo";
   
    io::stdin()
        .read_line(&mut guess);

    println!("You guessed: {guess}");
}
