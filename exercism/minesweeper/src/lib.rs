pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let columns: usize = minefield.len();
    let rows: usize = minefield[0].len();

    let matrix: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    // Transform the matrix into a vector of strings
    let matrix_as_strings: Vec<String> = matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|&val| val.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect();
    matrix_as_strings
}
