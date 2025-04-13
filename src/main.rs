mod game;

fn main() {
    let board = game::Board::new();
    let (player1, player2) = (
        game::Player { name: "ordinateur" },
        game::Player { name: "vincent" },
    );

    board.random_placement(player1);
    
    board.display(&player1);
}
