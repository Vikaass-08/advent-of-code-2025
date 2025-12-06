use std::fs;

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

fn find_invalid_ids(id_range_list: Vec<&str>) -> Result<Vec<i128>, String> {
    let mut result: Vec<i128> = Vec::new();

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


fn find_pattern(digits: usize, left_limit: i128, right_limit: i128, res: &mut Vec<i128>){
  if digits % 2 != 0 {
    return;
  }
  let first_half_size: usize = digits / 2;
  for digit in 1..=9 {
    self::find_digit(first_half_size - 1, digit as i128)
      .into_iter()
      .for_each(|val| {
        let multiplier: i128 = i128::pow(10, first_half_size as u32);
        let first_part = val * multiplier;
        let new_val: i128 = first_part + val;
        if new_val >= left_limit && new_val <= right_limit {
          res.push(new_val);
        }
      });
  }
}

fn find_digit(remainging_digits: usize, curr_val: i128) -> Vec<i128> {
  if remainging_digits <= 0 {
    return vec![curr_val];
  }
  let mut result: Vec<i128> = Vec::new();

  for digit in 0..=9 {
    let next_val = curr_val * 10 + digit as i128;
    let mut res = self::find_digit(remainging_digits - 1, next_val);
    result.append(&mut res);
  }
  return result;
}