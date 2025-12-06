mod day_one;
mod day_two;

fn main() {
    let part_two_response = day_two::part_two::solution();
    match part_two_response {
        Ok(res) => println!("Day Two - Part One Solution: {}", res),
        Err(err) => println!("Failed to process: {}", err)
    }
}
