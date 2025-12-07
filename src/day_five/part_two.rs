use std::{cmp::max, fs};

pub fn solution() -> Result<i128, String> {
  let file_path = "./src/static/input_part_five.txt";
  let file_content = fs::read_to_string(file_path).map_err(|err| err.to_string())?;
  let ingrediants: Vec<&str> = file_content.split('\n').collect();


 let mut range: Vec<Vec<i128>> = ingrediants
    .iter().filter(|v| v.contains("-"))
    .map(|range| {
      return range
        .split('-')
        .map(|v| v.parse::<i128>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<i128>, _>>()
    })
    .collect::<Result<_, _>>()?;

  range.sort_by(|a, b| a[0].cmp(&b[0]));
  
  let mut last_added: i128 = -1;
  let mut count = 0;

  range.iter().for_each(|range|{
    if range[1] <= last_added {
      return;
    }
    let left_limit = max(last_added + 1, range[0]);
    count += range[1] - left_limit + 1;
    last_added = range[1];
  });

  Ok(count)
}