use std::{collections::HashSet, fs};


pub fn solution() -> Result<i128, String> {
  let file_path = "./src/static/input_part_two.txt";
  let file_content = fs::read_to_string(file_path).map_err(|err| err.to_string())?;
  let id_range_list: Vec<&str> = file_content.split(',').collect();
  let result = self::find_invalid_ids(id_range_list)?;
  let res: i128 = result
    .into_iter()
    .reduce(|acc: i128, invalid_id: i128| acc + (invalid_id as i128))
    .unwrap_or(0);

  return Ok(res);
}

fn find_invalid_ids(id_range_list: Vec<&str>) -> Result<HashSet<i128>, String> {
    let mut result: HashSet<i128> = HashSet::new();

    for id_range in id_range_list {
      let parts: Vec<&str> = id_range.split('-').collect();

      // Expect exactly 2 parts: [left, right]
      let [left, right] = parts.as_slice() else {
        return Err("Failed to read values".into());
      };

      let left_limit: i128 = left.parse::<i128>().map_err(|e| e.to_string())?;
      let right_limit: i128 = right.parse::<i128>().map_err(|e| e.to_string())?;

      let left_digits: usize = left.len();
      let right_digits: usize = right.len();

      // Loop from number of digits of left → right
      for digits in left_digits..=right_digits {
        self::find_pattern(digits, left_limit, right_limit, &mut result);
      }
    }

    Ok(result)
}


fn find_pattern(digits: usize, left_limit: i128, right_limit: i128, res: &mut HashSet<i128>){
  
  self::find_all_divisors(digits)
    .into_iter()
    .for_each(|divisor| {
      for digit in 1..=9 {
        self::find_digit(divisor - 1, digit as i128)
          .into_iter()
          .for_each(|val: i128| {
            let multiplier: i128 = i128::pow(10, divisor as u32);
            let mut new_value: i128 = val;

            for _ in 1..(digits/divisor) {
              new_value = (new_value * multiplier) + val;
            }

            if new_value >= left_limit && new_value <= right_limit {
              res.insert(new_value);
            }

          });
      }
    });
}

fn find_digit(remainging_digits: usize, curr_val: i128) -> Vec<i128> {
  if remainging_digits <= 0 {
    return vec![curr_val];
  }
  let mut result: Vec<i128> = Vec::new();

  for digit in 0..=9 {
    let next_val = curr_val * 10 + digit as i128;
    let mut res: Vec<i128> = self::find_digit(remainging_digits - 1, next_val);
    result.append(&mut res);
  }
  return result;
}


fn find_all_divisors(digit: usize) -> Vec<usize> {

  (1..digit).into_iter()
    .filter(|x| digit % x == 0)
    .collect()
}