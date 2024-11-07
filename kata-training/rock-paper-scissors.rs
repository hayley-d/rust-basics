/*
"scissors", "paper" --> "Player 1 won!"
"scissors", "rock" --> "Player 2 won!"
"paper", "paper" --> "Draw!"
*/

fn rps(p1: &str, p2: &str) -> &'static str  {
    let winning_moves = [
        ("rock", "scissors"), 
        ("scissors", "paper"),
        ("paper", "rock"), 
    ];
    
    if winning_moves.contains(&(p1,p2)){
        return "Player 1 won!";
    } else if winning_moves.contains(&(p2,p1)) {
        return "Player 2 won!";
    }
    "Draw!"
}
