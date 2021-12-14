use std::fs;

fn main() {
    println!("PART ONE: BINGO");
    part_one();
    println!("PART TWO: SQUID WINS");
    part_two();
}

fn part_one() {
    let input = read_input(Some(false));
    let mut lines = input.lines();
    let numbers = parse_numbers(&mut lines);
    let mut boards = parse_boards(&mut lines);
    // println!("Numbers: {:?}", &numbers);
    // println!("Boards: {:?}", &boards);

    let mut found_winner = false;
    for num in numbers {
        for i in 0..boards.len() {
            boards[i].find(num);
        }
        for i in 0..boards.len() {
            if found_winner {
                break;
            }
            if boards[i].has_won() {
                println!(
                    "Winner is board {} with score: {}",
                    i + 1,
                    boards[i].sum_unmarked * num as u32
                );
                found_winner = true;
                break;
            }
        }
        if found_winner {
            break;
        }
    }
}

fn part_two() {
    let input = read_input(Some(true
    ));
    let mut lines = input.lines();
    let numbers = parse_numbers(&mut lines);
    let mut boards = parse_boards(&mut lines);
    // println!("Numbers: {:?}", &numbers);
    // println!("Boards: {:?}", &boards);

    let mut board_indices: Vec<usize> = (0..boards.len()).map(|i| i).collect();
    let mut num_winners = 0;
    for num in numbers {
        for i in board_indices.clone() {
            boards[i].find(num);
        }

        let mut should_remove = Vec::new();
        for i in 0..board_indices.len() {
            let board_index = board_indices[i];
            if boards[board_index].has_won() {
                should_remove.push(i);
                num_winners += 1;
                if num_winners == boards.len() {
                    println!(
                        "Winner is board {} with score: {}",
                        board_index + 1,
                        boards[board_index].sum_unmarked * num as u32
                    );
                }
            }
        }
        should_remove.sort();
        should_remove.reverse();
        for to_remove in should_remove {
            board_indices.remove(to_remove);
        }
    }
}

fn read_input(example: Option<bool>) -> String {
    let read_example = example.unwrap_or(false);
    let mut filename = "src/input.txt";
    if read_example {
        filename = "src/example.txt"
    }
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn parse_numbers(lines: &mut std::str::Lines) -> Vec<u16> {
    // let mut lines = input.lines();
    let numbers: Vec<u16> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u16>().unwrap())
        .collect();
    numbers
}

#[derive(Debug, Clone)]
struct B {
    board: Board,
    sum_unmarked: u32,
    tracker: [u8; 10],
}
impl B {
    fn new(board: Board) -> Self {
        let mut sum = 0;
        for r in 0..5 {
            for c in 0..5 {
                sum += board[r][c];
            }
        }
        Self {
            board,
            sum_unmarked: sum as u32,
            tracker: [0u8; 10],
        }
    }
    fn find(&mut self, num: u16) {
        for r in 0..5 {
            for c in 0..5 {
                if self.board[r][c] == num {
                    self.tracker[r] += 1;
                    self.tracker[c + 5] += 1;
                    self.sum_unmarked -= num as u32;
                }
            }
        }
    }
    fn has_won(&self) -> bool {
        for i in 0..self.tracker.len() {
            if self.tracker[i] == 5 {
                return true;
            }
        }
        false
    }
}
type Board = Vec<Vec<u16>>;
fn parse_board(lines: &mut std::str::Lines) -> Board {
    let mut board: Vec<Vec<u16>> = Vec::new();

    for _ in 0..5 {
        let line = lines.next().unwrap();
        let row = line
            .trim()
            .split_whitespace()
            .map(|num| num.trim().parse::<u16>().unwrap())
            .collect();
        board.push(row);
    }
    board
}

fn parse_boards(lines: &mut std::str::Lines) -> Vec<B> {
    let mut boards = Vec::new();

    while let Some(line) = lines.next() {
        if line == "" {
            let board = parse_board(lines);
            boards.push(B::new(board));
        }
    }

    boards
}
