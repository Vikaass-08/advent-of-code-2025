use std::fs;

pub fn solution() -> Result<usize, String> {
  let file_path = "./src/static/input_part_five.txt";
  let file_content = fs::read_to_string(file_path).map_err(|err| err.to_string())?;
  let ingrediants: Vec<&str> = file_content.split('\n').collect();


 let range: Vec<Vec<i128>> = ingrediants
    .iter().filter(|v| v.contains("-"))
    .map(|range| {
      return range
        .split('-')
        .map(|v| v.parse::<i128>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<i128>, _>>()
    })
    .collect::<Result<_, _>>()?;

  let fresh_ingrediants_range: Vec<i128> =
    range.into_iter().flatten().collect();

  let available_ingredients_range: Vec<i128> = ingrediants
    .iter().filter(|v| !v.contains("-") && v.len() > 0)
    .map(|v| v.parse::<i128>().map_err(|e| e.to_string()))
    .collect::<Result<_, _>>()?;

  let total = available_ingredients_range
    .into_iter()
    .filter(|val| {
      for i in (0..fresh_ingrediants_range.len()).step_by(2) {
        if val.ge(&fresh_ingrediants_range[i]) && val.le(&fresh_ingrediants_range[i + 1]) {
          return true;
        }
      }
      return false;
    })
    .count();

  Ok(total)
}