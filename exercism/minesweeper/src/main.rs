use minesweeper::annotate;

fn prepare_minefield(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| slice_to_string(r)).collect()
}

fn slice_to_string(row: &str) -> String {
    row.chars().collect()
}
fn main() {
    let minefield = &["   ", "   ", "   "];

    let minefield_updated = annotate(minefield);
    println!("{:#?}", minefield_updated);
}
