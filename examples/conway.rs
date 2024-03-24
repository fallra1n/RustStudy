use conway::{Cell, GameOfLife, Grid};

fn main() {
    let grid = Grid::from_slice(
        &[
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
        ],
        3,
        3,
    );
    let mut game = GameOfLife::from_grid(grid.clone());
    game.step();

    for i in 0..3 {
        for j in 0..3 {
            print!("{:?} ", game.get_grid().get(i, j));
        }
        println!()
    }
}
