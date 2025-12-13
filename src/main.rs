mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;

fn main() {
    let part_two_response = day_eight::part_two::solution();
    match part_two_response {
        Ok(res) => println!("Day Eight - Part Two Solution: {}", res),
        Err(err) => println!("Failed to process: {}", err)
    }
}
