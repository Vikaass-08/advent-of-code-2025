mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;

fn main() {
    let part_two_response = day_five::part_two::solution();
    match part_two_response {
        Ok(res) => println!("Day Five - Part Two Solution: {}", res),
        Err(err) => println!("Failed to process: {}", err)
    }
}
