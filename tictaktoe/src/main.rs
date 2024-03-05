use std::io;

fn who_won(string: &str){
    if string == "O"{
        println!("Player one WON!");
    } else {
        println!("Player two WON!");
    }
}
fn check_board(board : [[&str; 3]; 3]) -> bool {
    for i in 0..3 {
        if board[i][0] != " " && board[i][0] == board[i][1] &&  board[i][1] == board[i][2]{
            who_won(board[i][0]);
            return true;
        } else if board[0][i] != " " && board[0][i] == board[1][i] &&  board[1][i] == board[2][i]{
            who_won(board[0][i]);
            return true;
        }
    }
    if board[0][0] != " " && board[1][1] == board[0][0] && board[0][0] == board[2][2] {
        who_won(board[1][1]);
        return true;
    } else if board[0][2] != " " && board[1][1] == board[0][2] && board[1][1] == board[2][0] {
        who_won(board[1][1]);
        return true
    }
    return false;
}

fn main() {
    println!("User one: O \nUser two: X");
    let mut board = [[" ";3];3];
    let mut player_id = 0;
    let mut rounds_counter = 0;
    let players = ["O", "X"];

    loop{
        if rounds_counter >= 9 {
            println!("DRAW!!");
            break
        }
        println!("Type your move player {}", player_id + 1);

        let mut user_input = String::new();
        let x : usize;
        let y : usize;
        let _ = io::stdin().read_line(&mut user_input);
        x = user_input.chars().nth(0).unwrap() as usize - '0' as usize;
        y = user_input.chars().nth(1).unwrap() as usize - '0' as usize;


        if board[x][y] != " " {
            println!("Place taken, try again");
            continue
        }
        rounds_counter += 1;
        board[x][y] = players[player_id];

        player_id = (player_id+1) % 2;

        println!("{:?}", board[0]);
        println!("{:?}", board[1]);
        println!("{:?}", board[2]);
        if check_board(board)  {
            break;
        }
    }
}
