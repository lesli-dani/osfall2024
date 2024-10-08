// Assignment 2: Number Analyzer

// Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise.
fn is_even(n: i32) -> bool {

    if n % 2 == 0 {
        return true
    }
    return false
}

fn main() {
// Create an array of 10 integer numbers of your choice.
    let array:[i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];

// Use a for loop to iterate through the array and for each number:
    for &n in array.iter(){
        // Print whether it's even or odd using your is_even function
        if is_even(n){
            println!("{} is even!", n)
        }
        // If it's divisible by both 3 and 5, print "FizzBuzz"
        else if n%3 == 0 && n%5 == 0 {
            println!("FizzBuzz")
        }
        // If the number is divisible by 3, print "Fizz" instead
        else if n%3 == 0 {
            println!("Fizz")
        }
        // If the number is divisible by 5, print "Buzz" instead
        else if n%5 == 0 {
            println!("Buzz")
        }
        else {
            println!("{} is odd!", n)
        }
    }

// Use a while loop to find and print the sum of all numbers in the array.
    let mut count = 0;
    let mut total = 0;
    while count < 10{
        total += array[count];
        count += 1;
    }
    println!("The sum of all of the numbers is: {}", total);
    
// Use a loop to find and print the largest number in the array
    count = 0;
    let mut max = 0;
    loop {
        if array[count] > max{
            max = array[count];
        }
        count += 1;
        if count == 10{
            break;
        }
    }
    println!("The largest number is: {}", max);
}
