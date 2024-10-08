// Assignment 3: Guessing Game


// Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
fn check_guess(guess: i32, secret: i32) -> i32{
    // 0 if the guess is correct
    if guess == secret{
        return 0;
    }
    // 1 if the guess is too high
    else if guess > secret{
        return 1
    }
    // -1 if the guess is too low
    else {
        return -1
    }
}

// In the main function:
fn main() {
// Use a mutable variable to store a "secret" number (you can hard-code this).
    //let mut secret_num = 9;
    let secret_num = 9;
    
    // Set a mutable guess variable to a number of your choice (simulating user input)
    let mut guesses = 15;
// Use a loop to repeatedly:
    let mut count = 0;
    loop {
        count += 1;
        // Call the check_guess function
        // Use an if-else expression to print whether the guess was correct, too high, or too low
        // If the guess was correct, break the loop
        let checked = check_guess(guesses, secret_num);
        
        if checked == 0{
            println!("Guess was correct!");
            break;
        }
        else if checked == 1{
            println!("Guess was too high!");
            guesses -= 1;
        }
        else{
            println!("Guess was too low!");
            guesses += 1;
        }
    }
        // After the loop ends, print how many guesses it took (you'll need to track this in a variable)
    println!("Yay! You got it! It only took you {} guesses.", count);
}
