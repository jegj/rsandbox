const MOVES: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const MINE: u32 = 999999;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows: i32 = minefield.len() as i32;
    let columns: i32 = minefield[0].len() as i32;

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; columns as usize]; rows as usize];
    let mut mines: Vec<(i32, i32)> = Vec::new();

    for x in 0..minefield.len() {
        let row = minefield[x];
        let mut y = 0;
        for target in row.as_bytes() {
            if *target as i32 == 42 {
                println!("==============>mine => {} {}", x, y);
                let adjacents = generate_adjacents(x as i32, y, rows, columns);
                println!("adjacents =>{:?}", adjacents);
                add_mines(adjacents, &mut matrix);
                mines.push((x as i32, y));
            }
            y += 1;
        }
    }

    print_mines(mines, &mut matrix);

    //println!("result=>{:?}", matrix);

    fn print_element(elem: u32) -> String {
        let mine: String = "*".to_string();
        let empty: String = "·".to_string();

        match elem {
            0 => empty,
            MINE => mine,
            _ => elem.to_string(),
        }
    }

    // Transform the matrix into a vector of strings
    let matrix_as_strings: Vec<String> = matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|&val| print_element(val))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect();
    matrix_as_strings
}

pub fn add_mines(coordinates: Vec<(i32, i32)>, matrix: &mut Vec<Vec<u32>>) {
    for &(x, y) in &coordinates {
        matrix[x as usize][y as usize] += 1;
    }
}

pub fn generate_adjacents(x: i32, y: i32, x_len: i32, y_len: i32) -> Vec<(i32, i32)> {
    let mut adjacents: Vec<(i32, i32)> = Vec::new();
    for &(x_move, y_move) in MOVES {
        if valid_coordinates(x + x_move, y + y_move, x_len, y_len) {
            adjacents.push((x + x_move, y + y_move));
        }
    }
    adjacents
}

pub fn valid_coordinates(x: i32, y: i32, x_len: i32, y_len: i32) -> bool {
    println!("coord==> {} {} -> {} {}", x, y, x_len, y_len);
    x >= 0 && x < x_len && y >= 0 && y < y_len
}

pub fn print_mines(mines: Vec<(i32, i32)>, matrix: &mut Vec<Vec<u32>>) {
    for &(x, y) in &mines {
        matrix[x as usize][y as usize] = MINE as u32;
    }
}
