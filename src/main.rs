mod game;

fn main() {
    let (player1, player2) = (
        game::Player { name: "ordinateur" },
        game::Player { name: "vincent" },
    );

    let mut board1 = game::Board::new(player1);
    board1.random_placement();

    board1.display(&player1);
}
