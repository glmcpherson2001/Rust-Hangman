use std::io;
use crossterm::{execute, terminal};

fn main() {
    println!("Hello, Welcome to Hangman!\n");
    let mut correct_guesses: Vec<char> = Vec::new();
    let mut incorrect_guesses: Vec<char> = Vec::new();
    let random_phrase = get_random_phrase();

    loop {

        let underline_phrase = turn_phrase_to_underlines(&random_phrase, &correct_guesses);

        if underline_phrase.eq(&random_phrase){
            println!("You Win the phrase is '{}'", random_phrase);
            break;
        }

        println!("Guessed Letters: {:?}", incorrect_guesses);
        println!("{}", underline_phrase);

        println!("\n\n");


        let string_guess = get_user_input("Guess a Letter: ".to_string());

        let guessed_letter = string_guess.to_lowercase().chars().next();

        match guessed_letter {
            Some(guess) if guess.is_alphabetic() => {

                if correct_guesses.contains(&guess) || incorrect_guesses.contains(&guess){
                    println!("You've already guess '{}'", guess)

                }
                else if random_phrase.to_lowercase().contains(guess) {
                    println!("That's Correct! {} is in the word\n", guess);
                    correct_guesses.push(guess);
        
                } else {
                    println!("Unforntunately {} is not in the word\n", guess);
                    incorrect_guesses.push(guess);
                }
            } 
            Some(_) | None => {
                print!("Please Provide a letter!\n")
            }
        }

        // Clear the console
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    }
}


fn get_random_phrase() -> String {
    "Random Word".to_string()
}

fn turn_phrase_to_underlines(phrase: &String, correct_letters: &Vec<char>) -> String {
    let mut underline_string: Vec<char> = Vec::new();
    let phase_vec: Vec<char> = phrase.chars().collect();
    let space = ' ';

    for char in phase_vec {
        
        if char == space {
            underline_string.push(space);
            continue;
        }
        let lowercase_char = char.to_lowercase().next().unwrap();

         if correct_letters.contains(&lowercase_char) {
                 underline_string.push(char)
         } else {
                 underline_string.push('_')
         }
    }

    underline_string.into_iter().collect()
}

fn get_user_input(message: String) -> String {
    let mut user_input: String = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    return  user_input;
}