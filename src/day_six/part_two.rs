use std::{cmp, fs};

pub fn solution() -> Result<i128, String> {
  let file_path = "./src/static/input_part_six.txt";
  let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

  let lines: Vec<&str> = file_content.lines().collect();
  let last_line = lines.last().ok_or("Something went wrong")?;

  let opeation_per_column: Vec<&str> = last_line.trim().split_whitespace().collect();

  let mut chars_count_per_col: Vec<usize> = Vec::new();

  let mut last_added_index = 0;


  for (idx, c) in last_line.chars().enumerate() {
    if idx == 0 {
      continue;
    }
    else if c != ' ' {
      chars_count_per_col.push(idx - last_added_index - 1);
      last_added_index = idx;
    }
  }

  if last_added_index < last_line.len() {
    chars_count_per_col.push(last_line.len() - last_added_index);
  }

  let mut matrix: Vec<Vec<&str>> = Vec::new();

  for (idx, line) in lines.iter().enumerate() {
    // Skip last line
    if idx == lines.len() - 1 {
        continue;
    }

    matrix.push(Vec::new());
    let len: usize = matrix.len();


    let mut parts: Vec<&str> = self::split_single_space(line, &chars_count_per_col);

    matrix[len - 1].append(&mut parts);

  }

  let mut sum: i128 = 0;
  for col in 0..matrix[0].len() {
    let mut column = Vec::new();
    for row in 0..matrix.len() {
      column.push(matrix[row][col]);
    }
    sum += self::eval_column(&column, opeation_per_column[col])?;
  }
  Ok(sum)
}


fn split_single_space<'a>(s: &'a str, char_per_col: &Vec<usize>) -> Vec<&'a str> {
    let mut parts: Vec<&str> = Vec::new();

    let mut added = 0;
    let mut idx = 0;

    loop {
      if idx >= s.len() {
        break;
      }
      let end_idx = idx + char_per_col[added];
      parts.push(&s[idx..end_idx]);
      idx += char_per_col[added] + 1;
      added += 1;
    }


    parts
}


fn eval_column(column: &Vec<&str>, operation: &str) -> Result<i128, String> {


  let max_len = column.iter().fold(0, |acc, element| cmp::max(acc, element.len()));

  let mut numbers = vec![String::from(""); max_len];

  for val in column {
    val
      .chars()
      .enumerate()
      .for_each(|(idx, c)| {
        if c == ' ' {
          return;
        }
        numbers[idx].push(c)
      });
  }

  let nums = numbers.iter()
    .map(|val| val.parse::<i128>().map_err(|err| err.to_string()))
    .collect::<Result<Vec<i128>, String>>()?;


  if operation == "+" {
    let sum: i128 = nums.iter().sum();
    return Ok(sum); 
  }

  let res = nums.into_iter().fold(1_i128, |acc, val| acc * val);


  return Ok(res);
}
