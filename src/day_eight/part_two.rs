use std::{cmp::max, fs, usize};

pub fn solution() -> Result<i128, String> {
  let file_path = "./src/static/input_part_eight.txt";
  let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

  let positions: Vec<Vec<i32>> = file_content
    .lines()
    .into_iter()
    .map(|dimentions| dimentions
        .split(",")
        .map(|dir| dir.parse::<i32>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<i32>, String>>()
    )
    .collect::<Result<Vec<Vec<i32>>, String>>()?;

  let closest_pairs: Vec<(usize, usize, f64)> = get_closest_pairs_in_order(&positions);

  let mut parent: Vec<usize> = (0..positions.len()).map(|val| val).collect();
  let mut size: Vec<i128> = vec![1; parent.len()];

  let mut sum: i128 = 0;

  for idx in 0..closest_pairs.len() {
    let formed_union = form_union(closest_pairs[idx].0, closest_pairs[idx].1, &mut parent, &mut size);
    if formed_union {
      sum = positions[closest_pairs[idx].0][0] as i128 * positions[closest_pairs[idx].1][0] as i128
    }
  }
  
  Ok(sum)
}


fn form_union(d1: usize, d2: usize, parent: &mut Vec<usize>, size: &mut Vec<i128>) -> bool {
  let first_idx: usize = find_parent(d1, parent);
  let second_idx: usize = find_parent(d2, parent);

  if first_idx == second_idx {
    return false;
  }

  if size[second_idx] > size[first_idx] {
    parent[first_idx] = second_idx;
    size[second_idx] += size[first_idx];
    size[first_idx] = 0;
  }
  else {
    parent[second_idx] = first_idx;
    size[first_idx] += size[second_idx];
    size[second_idx] = 0;
  }

  return true;
}

fn find_parent(idx: usize, parent: &mut Vec<usize>) -> usize {
  if parent[idx] == idx {
    return idx;
  }

  parent[idx] = find_parent(parent[idx], parent);

  return parent[idx];
}

fn get_closest_pairs_in_order(positions: &Vec<Vec<i32>>) -> Vec<(usize, usize, f64)> {
  let mut pairs: Vec<(usize, usize, f64)> = Vec::new();

  for d1 in 0..positions.len() {
    for d2 in (d1 + 1)..positions.len() {
      let distance = get_euclidean_distance(&positions[d1], &positions[d2]);
      pairs.push((d1, d2, distance));
    }
  }

  pairs.sort_by(|a, b| a.2.total_cmp(&b.2));

  return pairs;

}

fn get_euclidean_distance(d1: &Vec<i32>, d2: &Vec<i32>) -> f64 {
  let first = (d1[0] as i64 - d2[0] as i64).pow(2);
  let second = (d1[1] as i64 - d2[1] as i64).pow(2);
  let third:i64  = (d1[2] as i64 - d2[2] as i64).pow(2);

  let sum = first + second + third;

  return (sum as f64).sqrt();
}