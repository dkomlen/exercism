pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if !input.is_empty() && !input[0].is_empty() {
        let max_points = input.iter().enumerate().map(|(r, row)| (r, row.iter().max().unwrap())).collect::<Vec<_>>();
        (0..input[0].len()).flat_map(|c| {
            let min = input.iter().map(|row| row[c]).min().unwrap();
            input.iter().enumerate().map(|(r, row)| (r, row[c])).filter(|(r, v)| *v == min && max_points.contains(&(*r,v))).map(|(r,_)| (r, c)).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    } else {
        Vec::new()
    }
}
