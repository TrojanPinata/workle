use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

const WORD_LIST_PATH: &str = "./possible.txt";

// load word list into vector
fn load_word_list(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()?;
    Ok(lines)
}

//
fn print_intro() {
    println!("This is workle, the third generation of my wordle solvers");
    println!("To use, type each guess and result as a entry in the format");
    println!("\n    guess = result\n\n        or\n\n    xxxxx = ooooo\n");
    println!("If a guess gives you a green letter, signify it as 'o' in the result");
    println!("if it is yellow, use \'y\' and grey (a miss) use \'x\'");
    println!("\nFor example:\n\n   caged = xoxyo\n");
    println!("In this case the \'a\' and \'d\' were all correct, \nthe \'e\' is in the wrong place, and the \'c\' and \'g\' were not\n");
    println!("To view your previously entered guesses enter \'guesses\' \nand clear them with \'clear\'");
    println!("\'quit\' will exit the program and \n\'solve\' will print the possible solutions as well as the program\'s best guess\n");
}

// solve and print best guess at solution
fn solve(guesses: &Vec<Vec<String>>, words: &Vec<String>) -> io::Result<()> {   // doesn't need to send anything back to main, just status
    let start = Instant::now();
    let mut confirmed: Vec<String> = vec![String::from("_"), String::from("_"), String::from("_"), String::from("_"), String::from("_")];
    let mut solutions: Vec<String> = vec![];
    let mut ordered: Vec<String> = vec![];
    let possible: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let mut possible_strings: Vec<String> = possible.iter().map(|&s| s.to_string()).collect();  // I absolutely hate that this is the solution to this
    for guess in guesses {
        for index in 0..5 {    // required to be exactly 2 long to be added to guesses vector
            let result_slice: String = guess[1][index..index+1].to_string();
            match result_slice.as_str() {
                "o" => {
                    confirmed[index] = guess[0][index..index+1].to_string();
                    ordered.retain(|x| x != &guess[0][index..index+1]);    // also add even if confirmed in case of duplicate
                    ordered.push(guess[0][index..index+1].to_string());
                },
                "y" => {
                    ordered.retain(|x| x != &guess[0][index..index+1]);    // delete from vector
                    ordered.push(guess[0][index..index+1].to_string());             // re add - prevents duplicates
                },
                "x" => {
                    let mut check = false;
                    for letter_index in 0..5 {  // handle edge case where user types character twice in single guess
                        if letter_index == index {
                            continue;
                        }
                        if &guess[0][index..index+1] == &guess[0][letter_index..letter_index+1] {
                            if &guess[1][letter_index..letter_index+1] == "y" { // case where both duplicates are grey, they need to be removed from possible list
                                check = true;
                                break;
                            }
                        }
                    }
                    if !check {
                        possible_strings.retain(|x| x != &guess[0][index..index+1]);
                    }
                },
                _ => (),
            }
        }
    }

    'by_word: for word in words {
        'by_letter: for index in 0..5 {
            let letter: String = word[index..index+1].to_string();
            if confirmed[index] != String::from("_") {  // there is a confirmed letter at this position
                if !(confirmed[index] == letter) {  // letter does not match confirmed letter
                    continue 'by_word;
                }
                continue 'by_letter;
            }
            else {  // no confirmed letter at this position, guess
                let mut check = false;
                'valid_letter: for poss in &possible_strings {
                    if letter == *poss {
                        check = true;       // letter found possible in list
                        break 'valid_letter;
                    }
                }
                if !check {
                    continue 'by_word;
                }
            }
        }

        let mut flag = false;
        'order_check: for order in &ordered { // recheck that each value in ordered can be found
            let mut check = false;
            'letter_check_ordered: for p in 0..5 {
                let letterp: String = word[p..p+1].to_string();
                if *order == letterp {
                    check = true;
                    break 'letter_check_ordered;
                }
            }
            if !check {
                flag = true;
                break 'order_check;
            }
        }
        if !flag {  // every letter in the ordered vector was found, add to solutions
            solutions.push(word.clone());
        }
    }

    // yellow guess cannot be in that position, so make sure it is not in final result
    let mut solutions_output = solutions.to_vec();
    for guess in guesses {
        for index in 0..5 {
            for solution in &solutions {
                let result_slice: String = guess[1][index..index+1].to_string();
                match result_slice.as_str() {
                    "y" => {
                        if solution[index..index+1].to_string() == guess[0][index..index+1].to_string() {
                            solutions_output.retain(|x| x != solution);
                        }
                    },
                    _ => (),
                }
            }
        }
    }

    if solutions_output.len() > 0 {
        if solutions_output.len() == 1 {
            println!("The solution is:\n"); // grammar
        }
        else {
            println!("There are {} possible solutions:\n", solutions_output.len());
        }
        for answer in solutions_output {
            println!("{}", answer);
        }
    }
    else {
        println!("No solution! Are you sure this is possible?");
    }

    let elapsed = start.elapsed();
    println!("\nSolved in {:.2?}\n", elapsed);

    Ok(())
}

// hey look, it's main!
fn main() -> io::Result<()> {
    let words = load_word_list(WORD_LIST_PATH)?;

    // handle inputs until solve or quit entered
    print_intro();
    let mut guesses: Vec<Vec<String>> = vec![];
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.as_str().trim();

        match input {
            "quit" => std::process::exit(0),
            "exit" => std::process::exit(0),
            "solve" => {
                println!();
                solve(&guesses, &words).ok();
            },
            "clear" => { 
                guesses.clear();
                println!("\nGuesses have been cleared\n");
            },
            "guesses" => {
                if guesses.len() == 0 {
                    println!("\nNo guesses yet. Guesses will be listed once entered\n");
                }
                else {
                    println!("\nGuesses so far:");
                    for item in &guesses  {
                        println!("{} = {}", item[0], item[1]);
                    }
                    println!();
                }
            },
            "" => (),
            _ => {
                // FIXME: move check to designated function
                let parts: Vec<String> = input.split('=').map(|s| s.trim()).map(String::from).collect();
                if parts.len() == 2 {
                    if parts[0].len() > 5 {
                        println!("Too many characters in guess, try again");
                    }
                    else if parts[0].len() < 5 {
                        println!("Too few characters in guess, try again");
                    }

                    if parts[1].len() > 5 {
                        println!("Too many characters in result, try again");
                    }
                    else if parts[1].len() < 5 {
                        println!("Too few characters in result, try again");
                    }

                    else {
                        let mut check = false;
                        for item in words.clone() {
                            if item == parts[0] {
                                check = true;
                                break;
                            }
                        }
                        if check {
                            for c in parts[1].chars() {
                                match c {
                                    'o' => (),
                                    'y' => (),
                                    'x' => (),
                                    _ => {
                                        check = false;
                                        break;
                                    },
                                }
                            }
                            if check {
                                guesses.push(parts.clone());
                            }
                            else {
                                println!("Result does not use legal characters, try again");
                            }
                        }
                        else {
                            println!("Guess is not legal word in wordle, try again");
                        }
                    }
                }
                else {
                    println!("Incorrect formatting used, make sure input is in either\n\'xxxxx = xxxxx\' format or a command as outlined above")
                }
                println!();
            },
        }
    }
}