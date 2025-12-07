mod day_one;
mod day_two;
mod day_three;
mod day_four;

fn main() {
    let part_two_response = day_four::part_two::solution();
    match part_two_response {
        Ok(res) => println!("Day Three - Part Two Solution: {}", res),
        Err(err) => println!("Failed to process: {}", err)
    }
}
