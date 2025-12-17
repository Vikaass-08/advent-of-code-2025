mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;
mod day_ten;

fn main() {
    let part_two_response = day_ten::part_one::solution();
    match part_two_response {
        Ok(res) => println!("Day Ten - Part One Solution: {}", res),
        Err(err) => println!("Failed to process: {}", err)
    }
}
