pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let columns: usize = minefield.len();
    let rows: usize = minefield[0].len();

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    for x in 0..minefield.len() {
        let row = minefield[x];
        let mut y = 0;
        for column in row.as_bytes() {
            let column_char = (*column as char).to_string();
            println!("row {} column==>{}", x, column_char);
            y += 1;
        }
    }

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
