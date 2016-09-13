use std::io;
use std::collections::HashMap;

fn main() {
    // Print start information.
    println!("Welcome to Cryptoquip!\nInput the puzzle:");

    // Get the puzzle in printable list based on column width. 
    let puzzle = get_puzzle();

    // Build guesses HashMap off of the letters in the puzzle.
    let mut guesses: HashMap<char, char> = HashMap::new();
    for line in puzzle.clone() {
    	for letter in line.chars() {
    		if letter.is_alphabetic() {
    			guesses.insert(letter, '_');
    		}
    	}
    }

    // Instantiate bool to know when to break out of loop.
    let mut complete = false;

    // Main game loop.
    while !complete {
        print_puzzle(&puzzle, &guesses);
        let g = guess();
        *guesses.get_mut(&g.0).unwrap() = g.1;
        complete = check_completed(&guesses);
    }

    // Puzzle is complete, print out message.
    println!("\nCongratulations you completed the puzzle!");
}

fn get_puzzle() -> Vec<String> {
	let mut puzzle = String::new();
    io::stdin().read_line(&mut puzzle).expect("Failed to read puzzle!");
    return sep_to_lines(puzzle);
}

fn sep_to_lines(puzzle: String) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();
	let words = puzzle.split(" ");

	lines.push("".to_string());

	for word in words {
		let line_len = lines.len()-1;
		if lines[line_len].len() + word.len() > 25 {
			lines.push(word.to_owned() + " ");
		} else {
			lines[line_len] = lines[line_len].to_owned() + word + " ";
		}
	}

	lines
}

fn print_puzzle(puzzle: &Vec<String>, guesses: &HashMap<char, char>) {
    println!("\n-------------------------\n");
    for line in puzzle {
        for letter in line.trim().chars() {
            match guesses.get(&letter) {
                Some(ans) => print!("{}", ans),
                None => print!("{}", letter)
            }
        }
        println!("\n{}\n", line);
    }
    println!("-------------------------\n");
}

fn guess() -> (char, char) {
    let mut input = String::new();

    println!("Enter the letter you want to guess:");
    io::stdin().read_line(&mut input).expect("Failed to read puzzle!");
    let letter_to_guess = input.chars().nth(0).unwrap();

    input = String::new();

    println!("Enter the letter you want replace {} with:", letter_to_guess);
    io::stdin().read_line(&mut input).expect("Failed to read puzzle!");
    let guess = input.chars().nth(0).unwrap();

    (letter_to_guess, guess)
}

fn check_completed(guesses: &HashMap<char, char>) -> bool {
    for (_, guess) in &guesses.clone() {
        if guess.eq(&'_') {
            return false;
        }
    }

    true
}