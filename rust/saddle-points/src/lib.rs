use std::cmp;
use std::collections::HashSet;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut sol = Vec::new();
    let mut max_indexes = HashSet::new();

    for (r, row) in input.iter().enumerate() {
        match row.iter().max() {
            Some(max) => {
                for (c, val) in row.iter().enumerate() {
                    if val == max {
                        max_indexes.insert((r,c));
                    }
                }
            }
            None => {}
        }
    }

    if !input.is_empty() {
        for c in 0..input[0].len() {
            let mut min = input[0][c];
            for row in input {
                min = cmp::min(min, row[c])
            }
            for (r, row) in input.iter().enumerate() {
                if row[c] == min && max_indexes.contains(&(r,c)) {
                    sol.push((r,c));
                }
            }
        }
    } 
    return sol;
}
