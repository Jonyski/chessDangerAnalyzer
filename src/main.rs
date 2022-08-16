use bunt;
use std::collections::HashMap;

fn main() {
    let board = vec!([" ", " ", " ", "p", " ", " ", " ", " "],
                     [" ", " ", " ", "N", " ", "p", " ", " "],
                     [" ", " ", "p", " ", "p", " ", "p", " "],
                     [" ", "p", " ", " ", " ", " ", " ", " "],
                     [" ", " ", "p", " ", " ", " ", "p", " "],
                     [" ", " ", " ", "q", " ", "b", " ", " "],
                     ["p", "p", "p", " ", " ", " ", "p", "p"],
                     ["t", " ", "b", " ", "k", "b", "n", "t"]
                     );

    let mut strToPieceName = HashMap::new();

    strToPieceName.insert("p", "pawn");
    strToPieceName.insert("q", "queen");
    strToPieceName.insert("t", "tower");
    strToPieceName.insert("b", "bishop");
    strToPieceName.insert("k", "king");
    strToPieceName.insert("n", "knight");

    let mut rowNumToLetter = HashMap::new();

    rowNumToLetter.insert(7 as usize, "a");
    rowNumToLetter.insert(6 as usize, "b");
    rowNumToLetter.insert(5 as usize, "c");
    rowNumToLetter.insert(4 as usize, "d");
    rowNumToLetter.insert(3 as usize, "e");
    rowNumToLetter.insert(2 as usize, "f");
    rowNumToLetter.insert(1 as usize, "g");
    rowNumToLetter.insert(0 as usize, "h");

    struct coord {
        line: usize,
        row: usize
    }

    impl coord {
        fn new(line: usize, row: usize) -> coord {
            coord {line, row}
        }
    }

    let printDangerMsg = |oponentPiece: String, formatedOponentCoord: String, myPiece: String, myPieceCoord: String| {
        bunt::println!("if the opponent moves his {[bold + #a1e2c6]} to {[bold]},\nhe will be able to eat your {[bold + #f1846c]} on {[bold]}", oponentPiece, formatedOponentCoord, myPiece, myPieceCoord);
    };



    let printBoardFormated = |piece: &str, beforeCoordinates: (usize, usize), afterCoordinates: (usize, usize), targetCoordinates: (usize, usize)| {
        for line in 0..8 as usize{
            for row in 0..8 as usize{
                if row == 0 {
                    bunt::print!("         {[#999999]} ", line + 1);
                }

                if (line, row) == beforeCoordinates {
                    bunt::print!("{$bold}#{/$} ");
                } else if (line, row) == afterCoordinates {
                    bunt::print!("{[#a1e2c6 + bold]} ", piece);
                } else if (line, row) == targetCoordinates {
                    bunt::print!("{[#f1846c + bold]} ", board[targetCoordinates.0][targetCoordinates.1]);
                } else {
                    bunt::print!("{} ", board[line][row]);
                }


                if row == 7 {
                    bunt::print!("\n");
                    if line == 7 {
                        bunt::println!("           {$#999999}h g f e d c b a{/$}");
                        bunt::println!("\n");
                    }
                }
            }
        }
    };

    let analyzePawnMoves = |line: usize, row: usize| {
        let afterCoords = (line + 1, row);
        if row < 7 {
            if board[line + 2][row + 1] != " " && board[line + 2][row + 1].chars().any(|c| matches!(c, 'a'..='z')) {

                let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&row).unwrap(), line + 2);
                let myPiece = strToPieceName.get(&board[line + 2][row + 1]).unwrap().to_string();
                let myPieceCoord = rowNumToLetter[&(row + 1)].to_string() + &(line + 3).to_string();
                
                printDangerMsg("pawn".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                printBoardFormated("P", (line, row), afterCoords, (line + 2, row + 1))
            }
        }

        if row > 0 {
            if board[line + 2][row - 1] != " " && board[line + 2][row - 1].chars().any(|c| matches!(c, 'a'..='z')) {

                let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&row).unwrap(), line + 2);
                let myPiece = strToPieceName.get(&board[line + 2][row - 1]).unwrap().to_string();
                let myPieceCoord = rowNumToLetter[&(row - 1)].to_string() + &(line + 3).to_string();

                printDangerMsg("pawn".to_string(), formatedOponentCoord, myPiece, myPieceCoord);
            }
            if board[line + 2][row - 1] != " " && board[line + 2][row - 1].chars().any(|c| matches!(c, 'a'..='z')) {
                printBoardFormated("P", (line, row), afterCoords, (line + 2, row - 1))
            }
        }
    };

    let createKnightOffsets = |line: usize, row: usize| -> Vec<(usize, usize)> {
        let mut knightOffsets = vec!();

        let offsets = vec!((2, -1),(2, 1),(-2, 1),(-2, -1),(1, 2),(-1, 2),(1, -2),(-1, -2));

        for offset in offsets {
            if line as i32 + offset.0 >= 0 && line as i32 + offset.0 < 8 && row as i32 + offset.1 >= 0 && row as i32 + offset.1 < 8{
                if offset.0 < 0 && offset.1 < 0{
                    knightOffsets.push((line - offset.0.abs() as usize, row - offset.1.abs() as usize));
                } else if offset.0 >= 0 && offset.1 < 0 {
                    knightOffsets.push((line + offset.0 as usize, row - offset.1.abs() as usize));
                } else if offset.0 < 0 && offset.1 >= 0{
                    knightOffsets.push((line - offset.0.abs() as usize, row + offset.1 as usize));
                } else {
                    knightOffsets.push((line + offset.0 as usize, row + offset.1 as usize));
                }

            } else {

            }
        }

        knightOffsets
    };

    let analyzeKnightDangers = |potentialMoves: Vec<(usize, usize)>, line: usize, row: usize| {
        println!("{:?}", potentialMoves);
        for potMov in potentialMoves {
            let potentialKnightDangers = createKnightOffsets(potMov.0, potMov.1);
            let knightDangers: Vec<&(usize, usize)> = potentialKnightDangers.iter().filter(|d| board[d.0][d.1].chars().any(|c| matches!(c, 'a'..='z') && d.0 >= 0 && d.0 < 8 && d.1 >= 0 && d.1 < 8)).collect();
            for danger in knightDangers {
                let formatedOponentCoord = format!("{}{}",rowNumToLetter.get(&potMov.1).unwrap(), potMov.0 + 1);
                let myPiece = strToPieceName.get(board[danger.0][danger.1]).unwrap().to_string();
                let myPieceCoord = format!("{}{}", rowNumToLetter.get(&danger.1).unwrap(), danger.0 + 1);

                printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                printBoardFormated("N", (line, row), (potMov.0, potMov.1), (danger.0, danger.1));

            }
        }
    };
    
    for line in 0..8 as usize{
        for row in 0..8 as usize{

            match board[line][row] {
                "P" => {
                    // moves are only analised if they are possible, duh
                    if (board[line + 1][row] == " ") {
                        analyzePawnMoves(line, row);
                    }
                },
                "T" => {
                    //analyzeTowerMoves(line, row);
                },
                "N" => {
                    let potentialMoves = createKnightOffsets(line, row);
                    analyzeKnightDangers(potentialMoves, line, row);
                },
                "B" => {

                },
                "Q" => {

                },
                "K" => {

                },
                _ => {

                }

            }
        }
    }
}
