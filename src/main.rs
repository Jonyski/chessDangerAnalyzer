use bunt;
use std::collections::HashMap;

fn main() {
    let board = vec!(["T", "N", "B", "Q", "K", "B", "N", "T"],
                     ["P", "P", "P", " ", " ", "P", "P", "P"],
                     [" ", " ", " ", " ", "P", " ", " ", " "],
                     [" ", " ", " ", " ", " ", " ", "N", " "],
                     [" ", " ", " ", "q", " ", " ", " ", " "],
                     [" ", " ", " ", "P", "p", " ", " ", " "],
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
                    bunt::print!("        ");
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
                        bunt::println!("\n\n");
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

    let analyzeKnightMoves = |line: usize, row: usize| {
        let mut potentialMoves: Vec<(usize, usize)> = vec!();

        if line > 1 {
            if row > 0 {
                if board[line - 2][row - 1] == " " {potentialMoves.push((line - 2, row - 1))};
            }
            if row < 7 {
                if board[line - 2][row + 1] == " " {potentialMoves.push((line - 2, row + 1))};
            }
        }

        if line < 6 {
            if row > 0 {
                if board[line + 2][row - 1] == " " {potentialMoves.push((line + 2, row - 1))};
            }
            if row < 7 {
                if board[line + 2][row + 1] == " " {potentialMoves.push((line + 2, row + 1))};
            }
        }

        if row > 1 {
            if line > 0 {
                if board[line - 1][row - 2] == " " {potentialMoves.push((line - 1, row - 2))};
            }
            if line < 7 {
                if board[line + 1][row - 2] == " " {potentialMoves.push((line + 1, row - 2))};
            }
        }

        if row < 6 {
            if line > 0 {
                if board[line - 1][row + 2] == " " {potentialMoves.push((line - 1, row + 2))};
            }
            if line < 7 {
                if board[line + 1][row + 2] == " " {potentialMoves.push((line + 1, row + 2))};
            }
        }

        let mut potentialAttacks: Vec<(usize, usize)> = vec!();

        for movement in potentialMoves {
            let newPosition = coord::new(movement.0, movement.1);


            if newPosition.line > 1 {
                if newPosition.row > 0 {
                    if board[newPosition.line - 2][newPosition.row - 1] != " " && board[newPosition.line - 2][newPosition.row - 1].chars().any(|c| matches!(c, 'a'..='z')) {

                        let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&(&newPosition.row)).unwrap(), newPosition.line + 1);
                        let myPiece = strToPieceName.get(&board[newPosition.line - 2][newPosition.row - 1]).unwrap().to_string();
                        let myPieceCoord = rowNumToLetter[&(newPosition.row - 1)].to_string() + &(newPosition.line - 2 + 1).to_string();

                        printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                        printBoardFormated("N", (line, row), (newPosition.line, newPosition.row), (newPosition.line - 2, newPosition.row - 1))
                    }
                }
                if newPosition.row < 7 {
                    if board[newPosition.line - 2][newPosition.row + 1] != " " && board[newPosition.line - 2][newPosition.row + 1].chars().any(|c| matches!(c, 'a'..='z')) {

                        let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&newPosition.row).unwrap(), newPosition.line + 1);
                        let myPiece = strToPieceName.get(&board[newPosition.line - 2][newPosition.row + 1]).unwrap().to_string();
                        let myPieceCoord = rowNumToLetter[&(newPosition.row + 1)].to_string() + &(newPosition.line - 2 + 1).to_string();

                        printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                        printBoardFormated("N", (line, row), (newPosition.line, newPosition.row), (newPosition.line - 2, newPosition.row + 1))
                    }
                }
            }

            if newPosition.line < 6 {
                if newPosition.row > 0 {
                    if board[newPosition.line + 2][newPosition.row - 1] != " " && board[newPosition.line + 2][newPosition.row - 1].chars().any(|c| matches!(c, 'a'..='z')) {

                        let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&newPosition.row).unwrap(), newPosition.line + 1);
                        let myPiece = strToPieceName.get(&board[newPosition.line + 2][newPosition.row - 1]).unwrap().to_string();
                        let myPieceCoord = rowNumToLetter[&(newPosition.row - 1)].to_string() + &(newPosition.line + 2 + 1).to_string();

                        printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                        printBoardFormated("N", (line, row), (newPosition.line, newPosition.row), (newPosition.line + 2, newPosition.row - 1))
                    }
                }
                if newPosition.row < 7 {
                    if board[newPosition.line + 2][newPosition.row + 1] != " " && board[newPosition.line + 2][newPosition.row + 1].chars().any(|c| matches!(c, 'a'..='z')) {

                        let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&newPosition.row).unwrap(), newPosition.line + 1);
                        let myPiece = strToPieceName.get(&board[newPosition.line + 2][newPosition.row + 1]).unwrap().to_string();
                        let myPieceCoord = rowNumToLetter[&(newPosition.row + 1)].to_string() + &(newPosition.line + 2 + 1).to_string();

                        printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                        printBoardFormated("N", (line, row), (newPosition.line, newPosition.row), (newPosition.line + 2, newPosition.row + 1))
                    }
                }
            }

            if newPosition.row > 1 {
                if newPosition.line > 0 {
                    if board[newPosition.line - 1][newPosition.row - 2] != " " && board[newPosition.line - 1][newPosition.row - 2].chars().any(|c| matches!(c, 'a'..='z')) {

                        let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&newPosition.row).unwrap(), newPosition.line + 1);
                        let myPiece = strToPieceName.get(&board[newPosition.line - 1][newPosition.row - 2]).unwrap().to_string();
                        let myPieceCoord = rowNumToLetter[&(newPosition.row - 2)].to_string() + &(newPosition.line - 1 + 1).to_string();

                        printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                        printBoardFormated("N", (line, row), (newPosition.line, newPosition.row), (newPosition.line - 1, newPosition.row - 2))
                    }
                }
                if newPosition.line < 7 {
                    if board[newPosition.line + 1][newPosition.row - 2] != " " && board[newPosition.line + 1][newPosition.row - 2].chars().any(|c| matches!(c, 'a'..='z')) {

                        let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&newPosition.row).unwrap(), newPosition.line + 1);
                        let myPiece = strToPieceName.get(&board[newPosition.line + 1][newPosition.row - 2]).unwrap().to_string();
                        let myPieceCoord = rowNumToLetter[&(newPosition.row - 2)].to_string() + &(newPosition.line + 1 + 1).to_string();

                        printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                        printBoardFormated("N", (line, row), (newPosition.line, newPosition.row), (newPosition.line + 1, newPosition.row - 2))
                    }
                }
            }

            if newPosition.row < 6 {
                if newPosition.line > 0 {
                    if board[newPosition.line - 1][newPosition.row + 2] != " " && board[newPosition.line - 1][newPosition.row + 2].chars().any(|c| matches!(c, 'a'..='z')) {

                        let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&newPosition.row).unwrap(), newPosition.line + 1);
                        let myPiece = strToPieceName.get(&board[newPosition.line - 1][newPosition.row + 2]).unwrap().to_string();
                        let myPieceCoord = rowNumToLetter[&(newPosition.row + 2)].to_string() + &(newPosition.line - 1 + 1).to_string();

                        printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                        printBoardFormated("N", (line, row), (newPosition.line, newPosition.row), (newPosition.line - 1, newPosition.row + 2))
                    }
                }
                if newPosition.line < 7 {
                    if board[newPosition.line + 1][newPosition.row + 2] != " " && board[newPosition.line + 1][newPosition.row + 2].chars().any(|c| matches!(c, 'a'..='z')) {

                        let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&newPosition.row).unwrap(), newPosition.line + 1);
                        let myPiece = strToPieceName.get(&board[newPosition.line + 1][newPosition.row + 2]).unwrap().to_string();
                        let myPieceCoord = rowNumToLetter[&(newPosition.row + 2)].to_string() + &(newPosition.line + 1 + 1).to_string();

                        printDangerMsg("knight".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                        printBoardFormated("N", (line, row), (newPosition.line, newPosition.row), (newPosition.line + 1, newPosition.row + 2))
                    }
                }
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
                    analyzeKnightMoves(line, row);
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
