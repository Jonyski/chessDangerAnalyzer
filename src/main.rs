use bunt;
use std::collections::HashMap;

#[allow(non_snake_case)]
#[allow(unused_comparisons)]
fn main() {
    /*  CHANGE THIS VECTOR TO THE BOARD THAT YOU WANT TO ANALYZE
        UPPERCASE LETTERS ARE THE OPONENT PIECES
        lowercase letters are your pieces
        K = KING
        Q = QUEEN
        R = ROOK
        B = BISHOP
        N = KNIGHT
        P = PAWN
    */
    let board = vec!(["K", " ", " ", "R", " ", " ", " ", "R"],
                     ["P", "P", " ", " ", " ", "P", "P", "P"],
                     [" ", " ", " ", "B", "P", "N", " ", " "],
                     [" ", " ", " ", " ", " ", " ", "n", " "],
                     [" ", " ", " ", "p", " ", "b", " ", " "],
                     [" ", " ", " ", " ", " ", " ", "p", "b"],
                     ["p", "p", " ", " ", "p", "p", " ", "p"],
                     [" ", "k", "r", "r", " ", " ", " ", " "]
                     );

    let mut strToPieceName = HashMap::new();

    strToPieceName.insert("p", "pawn");
    strToPieceName.insert("q", "queen");
    strToPieceName.insert("r", "rook");
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

    let analyzePawnMoves = |line: usize, row: usize, moveType: &str| {
        let afterCoords = (line + 1, row);
        if row < 7 {
            if board[line + 2][row + 1] != " " && board[line + 2][row + 1].chars().any(|c| matches!(c, 'a'..='z')) {

                let formatedOponentCoord = format!("{}{}", rowNumToLetter.get(&row).unwrap(), line + 2);
                let myPiece = strToPieceName.get(&board[line + 2][row + 1]).unwrap().to_string();
                let myPieceCoord = rowNumToLetter[&(row + 1)].to_string() + &(line + 3).to_string();
                
                printDangerMsg("pawn".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                if moveType == "single move" {
                    printBoardFormated("P", (line, row), afterCoords, (line + 2, row + 1));
                } else if moveType == "double move" {
                    printBoardFormated("P", (line - 1, row), afterCoords, (line + 2, row + 1));
                }
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
                if moveType == "single move" {
                    printBoardFormated("P", (line, row), afterCoords, (line + 2, row - 1));
                } else if moveType == "double move" {
                    printBoardFormated("P", (line - 1, row), afterCoords, (line + 2, row - 1));
                }
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

    let createRookOffsets = |line: usize, row: usize, offsetType: &str| -> Vec<(usize, usize)> {
        let mut rookOffsets = vec!();

        let mut offsets = vec!();

        for i in 1..8 {
            offsets.push((-i as i32, 0 as i32));
            offsets.push((0 as i32, i as i32));
            offsets.push((i as i32, 0 as i32));
            offsets.push((0 as i32, -i as i32));
        }

        let mut blockLeft = vec!();
        let mut blockUp = vec!();
        let mut blockRight = vec!();
        let mut blockDown = vec!();

        for offset in &offsets {
            if line as i32 + offset.0 >= 0 && line as i32 + offset.0 < 8 && row as i32 + offset.1 >= 0 && row as i32 + offset.1 < 8{
                if offset.1 < 0 && board[line][row - offset.1.abs() as usize] != " "{
                    blockLeft.push(offset.1);
                } else if offset.0 < 0 && board[line - offset.0.abs() as usize][row] != " "{
                    blockUp.push(offset.0);
                } else if offset.1 > 0 && board[line][row + offset.1 as usize] != " "{
                    blockRight.push(offset.1);
                } else if offset.0 > 0 && board[line + offset.0 as usize][row] != " "{
                    blockDown.push(offset.0);
                }
            }
        }

        let bl = *blockLeft.iter().max().unwrap_or(&-10);
        let bu = *blockUp.iter().max().unwrap_or(&-10);
        let br = *blockRight.iter().min().unwrap_or(&10);
        let bd = *blockDown.iter().min().unwrap_or(&10);


        if offsetType == "movement" {
            for offset in &offsets {
                if line as i32 + offset.0 >= 0 && line as i32 + offset.0 < 8 && row as i32 + offset.1 >= 0 && row as i32 + offset.1 < 8{
                    //moves left
                    if offset.1 < 0 && offset.1 > bl{
                        rookOffsets.push((line, row - offset.1.abs() as usize));
                    } 
                    //moves up
                    else if offset.0 < 0 && offset.0 > bu{
                        rookOffsets.push((line - offset.0.abs() as usize, row));
                    } 
                    //moves right
                    else if offset.1 > 0 && offset.1 < br{
                        rookOffsets.push((line, row + offset.1 as usize));
                    }
                    //moves down
                    else if offset.0 > 0 && offset.0 < bd{
                        rookOffsets.push((line + offset.0 as usize, row));
                    }
                }
            }
        } else if offsetType == "danger" {
            for offset in &offsets {
                if line as i32 + offset.0 >= 0 && line as i32 + offset.0 < 8 && row as i32 + offset.1 >= 0 && row as i32 + offset.1 < 8{
                    //moves left
                    if offset.1 < 0 && offset.1 >= bl{
                        rookOffsets.push((line + offset.0 as usize, row - offset.1.abs() as usize));
                    } 
                    //moves up
                    else if offset.0 < 0 && offset.0 >= bu{
                        rookOffsets.push((line - offset.0.abs() as usize, row + offset.1 as usize));
                    } 
                    //moves right
                    else if offset.1 > 0 && offset.1 <= br{
                        rookOffsets.push((line + offset.0 as usize, row + offset.1 as usize));
                    }
                    //moves down
                    else if offset.0 > 0 && offset.0 <= bd{
                        rookOffsets.push((line + offset.0 as usize, row + offset.1 as usize));
                    }
                }
            }
        }


        rookOffsets
    };

    let analyzeRookDangers = |potentialMoves: Vec<(usize, usize)>, line: usize, row: usize| {

        for potMov in potentialMoves {
            let potentialRookDangers = createRookOffsets(potMov.0, potMov.1, "danger");
            let rookDangers: Vec<&(usize, usize)> = potentialRookDangers.iter().filter(|d| board[d.0][d.1].chars().any(|c| matches!(c, 'a'..='z') && d.0 >= 0 && d.0 < 8 && d.1 >= 0 && d.1 < 8)).collect();

            for danger in rookDangers {
                let formatedOponentCoord = format!("{}{}",rowNumToLetter.get(&potMov.1).unwrap(), potMov.0 + 1);
                let myPiece = strToPieceName.get(board[danger.0][danger.1]).unwrap().to_string();
                let myPieceCoord = format!("{}{}", rowNumToLetter.get(&danger.1).unwrap(), danger.0 + 1);

                printDangerMsg("rook".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                printBoardFormated("R", (line, row), (potMov.0, potMov.1), (danger.0, danger.1));

            }
        }

    };

    let createBishopOffsets = |line: usize, row: usize, offsetType: &str| -> Vec<(usize, usize)> {
        let mut bishopOffsets = vec!();

        let mut offsets = vec!();

        for i in 1..8 {
            offsets.push((-i as i32, -i as i32));
            offsets.push((-i as i32, i as i32));
            offsets.push((i as i32, i as i32));
            offsets.push((i as i32, -i as i32));
        }

        let mut blockUpLeft = vec!();
        let mut blockUpRight = vec!();
        let mut blockDownRight = vec!();
        let mut blockDownLeft = vec!();

        for offset in &offsets {
            if line as i32 + offset.0 >= 0 && line as i32 + offset.0 < 8 && row as i32 + offset.1 >= 0 && row as i32 + offset.1 < 8{
                //up left
                if offset.0 < 0 && offset.1 < 0 && board[line - offset.0.abs() as usize][row - offset.1.abs() as usize] != " "{
                    blockUpLeft.push((offset.0, offset.1));
                }
                //up right 
                else if offset.0 < 0 && offset.1 > 0 && board[line - offset.0.abs() as usize][row + offset.1 as usize] != " "{
                    blockUpRight.push((offset.0, offset.1));
                } 
                //down right
                else if offset.0 > 0 && offset.1 > 0 && board[line + offset.0 as usize][row + offset.1 as usize] != " "{
                    blockDownRight.push((offset.0, offset.1));
                }
                //down left 
                else if offset.0 > 0 && offset.1 < 0 && board[line + offset.0 as usize][row - offset.1.abs() as usize] != " "{
                    blockDownLeft.push((offset.0, offset.1));
                }
            }
        }

        let bul = *blockUpLeft.iter().max().unwrap_or(&(-10, -10));
        let bur = *blockUpRight.iter().max().unwrap_or(&(-10, 10));
        let bdr = *blockDownRight.iter().min().unwrap_or(&(10, 10));
        let bdl = *blockDownLeft.iter().min().unwrap_or(&(10, -10));


        if offsetType == "movement" {
            for offset in &offsets {
                if line as i32 + offset.0 >= 0 && line as i32 + offset.0 < 8 && row as i32 + offset.1 >= 0 && row as i32 + offset.1 < 8{
                    //moves up left
                    if offset.0 < 0 && offset.1 < 0 && offset > &bul{
                        bishopOffsets.push((line - offset.0.abs() as usize, row - offset.1.abs() as usize));
                    } 
                    //moves up right
                    else if offset.0 < 0 && offset.1 > 0 && offset > &bur{
                        bishopOffsets.push((line - offset.0.abs() as usize, row + offset.1 as usize));
                    } 
                    //moves down right
                    else if offset.0 > 0 && offset.1 > 0 && offset < &bdr{
                        bishopOffsets.push((line + offset.0 as usize, row + offset.1 as usize));
                    }
                    //moves down left
                    else if offset.0 > 0 && offset.1 < 0 && offset < &bdl{
                        bishopOffsets.push((line + offset.0 as usize, row - offset.1.abs() as usize));
                    }
                }
            }
        } else if offsetType == "danger" {
            for offset in &offsets {
                if line as i32 + offset.0 >= 0 && line as i32 + offset.0 < 8 && row as i32 + offset.1 >= 0 && row as i32 + offset.1 < 8{
                    //moves up left
                    if offset.0 < 0 && offset.1 < 0 && offset >= &bul{
                        bishopOffsets.push((line - offset.0.abs() as usize, row - offset.1.abs() as usize));
                    } 
                    //moves up right
                    else if offset.0 < 0 && offset.1 > 0 && offset >= &bur{
                        bishopOffsets.push((line - offset.0.abs() as usize, row + offset.1 as usize));
                    } 
                    //moves down right
                    else if offset.0 > 0 && offset.1 > 0 && offset <= &bdr{
                        bishopOffsets.push((line + offset.0 as usize, row + offset.1 as usize));
                    }
                    //moves down left
                    else if offset.0 > 0 && offset.1 < 0 && offset <= &bdl{
                        bishopOffsets.push((line + offset.0 as usize, row - offset.1.abs() as usize));
                    }
                }
            }
        }


        bishopOffsets
    };

    let analyzeBishopDangers = |potentialMoves: Vec<(usize, usize)>, line: usize, row: usize| {

        for potMov in potentialMoves {
            let potentialBishopDangers = createBishopOffsets(potMov.0, potMov.1, "danger");
            let bishopDangers: Vec<&(usize, usize)> = potentialBishopDangers.iter().filter(|d| board[d.0][d.1].chars().any(|c| matches!(c, 'a'..='z') && d.0 >= 0 && d.0 < 8 && d.1 >= 0 && d.1 < 8)).collect();

            for danger in bishopDangers {
                let formatedOponentCoord = format!("{}{}",rowNumToLetter.get(&potMov.1).unwrap(), potMov.0 + 1);
                let myPiece = strToPieceName.get(board[danger.0][danger.1]).unwrap().to_string();
                let myPieceCoord = format!("{}{}", rowNumToLetter.get(&danger.1).unwrap(), danger.0 + 1);

                printDangerMsg("bishop".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                printBoardFormated("B", (line, row), (potMov.0, potMov.1), (danger.0, danger.1));

            }
        }

    };
    
    let analyzeQueenDangers = |potentialMoves: Vec<(usize, usize)>, line: usize, row: usize| {

        for potMov in potentialMoves {
            let mut potentialQueenDangers = createRookOffsets(potMov.0, potMov.1, "danger");
            let mut bishopLikeOffsets = createBishopOffsets(potMov.0, potMov.1, "danger");

            potentialQueenDangers.append(&mut bishopLikeOffsets);

            let queenDangers: Vec<&(usize, usize)> = potentialQueenDangers.iter().filter(|d| board[d.0][d.1].chars().any(|c| matches!(c, 'a'..='z') && d.0 >= 0 && d.0 < 8 && d.1 >= 0 && d.1 < 8)).collect();

            for danger in queenDangers {
                let formatedOponentCoord = format!("{}{}",rowNumToLetter.get(&potMov.1).unwrap(), potMov.0 + 1);
                let myPiece = strToPieceName.get(board[danger.0][danger.1]).unwrap().to_string();
                let myPieceCoord = format!("{}{}", rowNumToLetter.get(&danger.1).unwrap(), danger.0 + 1);

                printDangerMsg("queen".to_string(), formatedOponentCoord, myPiece, myPieceCoord);

                printBoardFormated("Q", (line, row), (potMov.0, potMov.1), (danger.0, danger.1));

            }
        }

    };

    for line in 0..8 as usize{
        for row in 0..8 as usize{

            match board[line][row] {
                "P" => {
                    // moves are only analised if they are possible, duh
                    if board[line + 1][row] == " " {
                        analyzePawnMoves(line, row, "single move");
                    }
                    if line == 1 && board[line + 2][row] == " " {
                        analyzePawnMoves(line + 1 as usize, row, "double move");
                    }
                },
                "R" => {
                    let potentialMoves = createRookOffsets(line, row, "movement");
                    analyzeRookDangers(potentialMoves, line, row);
                },
                "N" => {
                    let potentialMoves = createKnightOffsets(line, row);
                    analyzeKnightDangers(potentialMoves, line, row);
                },
                "B" => {
                    let potentialMoves = createBishopOffsets(line, row, "movement");
                    analyzeBishopDangers(potentialMoves, line, row);
                },
                "Q" => {
                    let mut potentialMoves = createRookOffsets(line, row, "movement");
                    let mut potentialMoves2 = createBishopOffsets(line, row, "movement");

                    potentialMoves.append(&mut potentialMoves2);

                    analyzeQueenDangers(potentialMoves, line, row);
                },
                _ => {

                }

            }
        }
    }
}
