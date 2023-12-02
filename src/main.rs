use core::fmt;
use std::{
    fmt::Display,
    io::{stdin, stdout, Write, Stdout}, process::exit,
};
use termion::{event::Key, raw::RawTerminal, cursor::DetectCursorPos};
use termion::{input::TermRead, raw::IntoRawMode};

fn show_board(board: &Board, f: &mut RawTerminal<Stdout>) {
    write!(f, "{}", board).unwrap();
    f.flush().unwrap();
}

fn main() {
    #[allow(non_snake_case)]
    let YELLOW = termion::color::Yellow.fg_str();
    #[allow(non_snake_case)]
    let RESET = termion::color::Reset.fg_str();
    #[allow(non_snake_case)]
    let CLEAR = termion::clear::All;
    #[allow(non_snake_case)]
    let CLEAR_BELOW = termion::clear::AfterCursor;
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut board = Board::new(4, 4);
    board.sprinkle_random();
    
    write!(
        stdout,
        "{}{}{}{}{}{}{}\r",
        termion::cursor::Hide,
        termion::cursor::Goto(1,1),
        CLEAR,
        YELLOW,
        "Welcome to 2^11!",
        RESET,
        termion::cursor::Goto(1,2),
    )
    .unwrap();

    stdout.flush().unwrap();
    

    let mut keys = stdin.keys().into_iter();

    keys.next();

    show_board(&board, &mut stdout);

    while let Some(Ok(key)) = keys.next() {
        let action = match key {
            Key::Esc => exit(0),
            Key::Ctrl('c') => exit(0),
            Key::Left => Action::Left,
            Key::Right => Action::Right,
            Key::Up => Action::Up,
            Key::Down => Action::Down,
            _ => continue,
        };
    // loop {
        
        // let action_index = rand::random::<usize>() % 4;
        // let action = &vec![Action::Up, Action::Down, Action::Left, Action::Right][action_index];


        write!(
            stdout,
            "{}{}\rScore: {}[{:0>6}]{}\n\r",
            termion::cursor::Goto(1,1),
            CLEAR_BELOW,
            YELLOW,
            board.total_score,
            RESET
        )
        .unwrap();

        board.step(&action);
        show_board(&board, &mut stdout);

        // thread::sleep(Duration::from_millis(100));

        write!(stdout, "{}{}", termion::cursor::Goto(1, 2),CLEAR_BELOW).unwrap();
        board.sprinkle_random();
        show_board(&board, &mut stdout);

        if board.lost() {
            break;
        } else {
            continue;
        }
    }

    writeln!(
        stdout,
        "\n\r{}{}",
        termion::color::Red.fg_str(),
        "You died bro"
    )
    .unwrap();
    write!(stdout, "{}", termion::color::Reset.fg_str()).unwrap();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    None,
    Some(u8),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            // Tile::None => Ok(()),
            Tile::None => Ok(()),
            Tile::Some(power) => write!(f, "{}", 2usize.pow(power as u32)),
        }
    }
}

enum Action {
    Up,
    Left,
    Down,
    Right,
}
struct Board {
    width: usize,
    height: usize,
    pub cells: Vec<Vec<Tile>>,
    filled_tiles: usize,
    total_score : usize,
}

impl Board {
    const BORDER_SYMBOL: &'static str = "##";
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell_width = 5;
        let wall_width = Board::BORDER_SYMBOL.len();
        let total_wall = 1 + (cell_width * self.width) / wall_width + 1;
        writeln!(f, "{}\r", Board::BORDER_SYMBOL.repeat(total_wall))?;
        writeln!(
            f,
            "{1}{0}{1}\r",
            " ".repeat(wall_width).repeat(total_wall - 2),
            Board::BORDER_SYMBOL
        )?;
        for h in 0..self.height {
            write!(f, "{}", Board::BORDER_SYMBOL)?;
            for w in 0..self.width {
                write!(
                    f,
                    "{value:^width$}",
                    value = format!("{}", self.cells[h][w]),
                    width = cell_width
                )?;
            }
            writeln!(f, "{}\r", Board::BORDER_SYMBOL)?;
            writeln!(
                f,
                "{1}{0}{1}\r",
                " ".repeat(wall_width).repeat(total_wall - 2),
                Board::BORDER_SYMBOL
            )?;
        }
        writeln!(f, "{}\r", Board::BORDER_SYMBOL.repeat(total_wall))?;
        Ok(())
    }
}

fn reduce_line(elems : impl Iterator<Item=Tile>, size : usize) -> (usize, impl Iterator<Item=Tile> + DoubleEndedIterator) {
    let mut buffer = Vec::with_capacity(size);
    let mut has_merged = false;
    let mut iter = elems.filter(|t| matches!(*t, Tile::Some(_))).peekable();
    let mut nonempty_added = 0;
    let mut score = 0;
    while let Some(tile) = iter.next() {
        let Tile::Some(power) = tile else {unreachable!()};
        nonempty_added += 1;
        if has_merged == false && Some(&tile) == iter.peek() {
            buffer.push(Tile::Some(power+1));
            has_merged = true;
            score = 2usize.pow(power as u32+1);
            let _ = iter.next(); // next has been merged so we discard
        } else {
            buffer.push(Tile::Some(power));
        }
        }
    
    buffer.extend(std::iter::repeat(Tile::None).take(size - nonempty_added));
    (score, buffer.into_iter())
}

trait Game2048: Display {
    fn new(width: usize, height: usize) -> Self;
    fn from(row_major_board : Vec<Vec<Tile>>) -> Self;
    fn get_data(&self) -> Vec<Vec<Tile>>;
    fn step(&mut self, action: &Action) -> &Self;
    fn sprinkle_random(&mut self) -> &Self;
    fn lost(&self) -> bool;
}

impl Game2048 for Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            cells: vec![vec![Tile::None; width]; height],
            filled_tiles: 0,
            total_score: 0,
        }
    }

    fn from(row_major_board : Vec<Vec<Tile>>) -> Board {
        let height = row_major_board.len();
        let width = row_major_board[0].len();
        let filled_tiles = row_major_board.iter().flatten().filter(|t| matches!(**t, Tile::Some(_))).count();
        Board { width, height, cells: row_major_board, filled_tiles , total_score:0}
    }

    fn sprinkle_random(&mut self) -> &Board {
        fn get_random() -> Tile {
            match rand::random::<usize>() % 100 {
                0..=79 => Tile::Some(1), // P(2) = 0.8
                _ => Tile::Some(2),      // P(4) = 0.2
            }
        }

        // choose a random index from all the empty tiles
        let index = rand::random::<usize>() % (self.width * self.height - self.filled_tiles);

        let cell_to_change = self
            .cells
            .iter_mut()
            .flatten()
            .filter(|t| !matches!(**t, Tile::Some(_)))
            .take(index + 1)
            .last()
            .unwrap();
        *cell_to_change = get_random();
        self.filled_tiles += 1;
        self
    }

    fn step(&mut self, action: &Action) -> &Board {

        // basically, scan from left to right and find the first pair that matches
        
        match action {
            &Action::Left => {
                for row in self.cells.iter_mut() {                    
                    let line_to_move = row.iter().copied();
                    let (score, new_row) = reduce_line(line_to_move, self.width);
                    *row = new_row.collect();
                    self.filled_tiles -= if score > 0 {1} else {0};
                    self.total_score += score;
                }
            }
            &Action::Right => {
                for row in self.cells.iter_mut() {                    
                    let line_to_move = row.iter().copied().rev();
                    let (score, new_row) = reduce_line(line_to_move, self.width);
                    *row = new_row.rev().collect();
                    self.filled_tiles -= if score > 0 {1} else {0};
                    self.total_score += score;
                }
            }
            &Action::Down => {
                for w in 0..self.width {
                    
                    let mut column : Vec<Tile> = Vec::with_capacity(self.height);
                    for h in 0..self.height {
                        column.push(self.cells[h][w])
                    }
                    let line_to_move = column.iter().copied().rev();
                    let (score, moved_line) = reduce_line(line_to_move, self.height);
                    for (h, moved_tile) in moved_line.rev().enumerate() {
                        self.cells[h][w] = moved_tile;
                    }
                    self.filled_tiles -= if score > 0 {1} else {0};
                    self.total_score += score;
                }
            }
            &Action::Up => {
                for w in 0..self.width {
                    
                    let mut column : Vec<Tile> = Vec::with_capacity(self.height);
                    for h in 0..self.height {
                        column.push(self.cells[h][w])
                    }
                    let line_to_move = column.iter().copied();
                    let (score, moved_line) = reduce_line(line_to_move, self.height);
                    for (h, moved_tile) in moved_line.enumerate() {
                        self.cells[h][w] = moved_tile;
                    }
                    self.filled_tiles -= if score > 0 {1} else {0};
                    self.total_score += score;
                }
            }
        }

        self
    }

    fn lost(&self) -> bool {
        self.filled_tiles == (self.height * self.width)
    }

    fn get_data(&self) -> Vec<Vec<Tile>> {
        self.cells.clone()
    }

}


#[cfg(test)]
mod tests {
    use crate::Board;

    use super::{Tile, Tile::None, Tile::Some, reduce_line, Game2048};
    #[test]
    fn reduce_line_test() {
        let cand = vec![Some(1), None];
        assert_eq!(cand, reduce_line(cand.clone().into_iter(), 2).1.collect::<Vec<Tile>>());
        
        let cand = vec![None, None];
        assert_eq!(cand, reduce_line(cand.clone().into_iter(), 2).1.collect::<Vec<Tile>>());
        
        let mut cand = vec![None, Some(1)];
        cand.reverse();
        let expected = cand.clone();
        assert_eq!(expected, reduce_line(cand.clone().into_iter(), 2).1.collect::<Vec<Tile>>());
        
        let cand = vec![Some(1), Some(1)];
        assert_eq!(vec![Some(2), None], reduce_line(cand.into_iter(), 2).1.collect::<Vec<Tile>>());
        
        let cand = vec![Some(1), None, Some(1)];
        
        assert_eq!(vec![Some(2), None, None], reduce_line(cand.into_iter(), 3).1.collect::<Vec<Tile>>());
        
        let cand = vec![Some(1), Some(1), Some(1)];
        assert_eq!(vec![Some(2), Some(1), None], reduce_line(cand.into_iter(), 3).1.collect::<Vec<Tile>>());
        
        let cand = vec![Some(1), Some(2), Some(1)];
        assert_eq!(vec![Some(1), Some(2), Some(1)], reduce_line(cand.into_iter(), 3).1.collect::<Vec<Tile>>());
    }
    
    fn game_tester_horizontal<G : Game2048>() {

        let board_data = vec! [
            vec![None, Some(1), None]
        ];

        let mut board = G::from(board_data);

        board.step(&crate::Action::Left);

        let expected_board = vec! [
            vec![Some(1), None, None]
        ];

        assert_eq!(expected_board, board.get_data());
        
        board.step(&crate::Action::Right);

        let expected_board = vec! [
            vec![None, None, Some(1)] //
        ];

        assert_eq!(expected_board, board.get_data());
    }

    fn game_tester_vertical<G : Game2048>() {

        let board_data = vec! [
            vec![None], //
            vec![Some(1)],
            vec![None],
        ];
        
        let mut board = G::from(board_data);
        
        board.step(&crate::Action::Down);
        
        let expected_board = vec! [
            vec![None], //
            vec![None],
            vec![Some(1)],
        ];

        assert_eq!(expected_board, board.get_data());
        
        board.step(&crate::Action::Up);

        let expected_board = vec! [
            vec![Some(1)], //
            vec![None],
            vec![None],
        ];

        assert_eq!(expected_board, board.get_data());
    }

    #[test]
    fn test_board() {
        game_tester_horizontal::<Board>();
        game_tester_vertical::<Board>();
    }
}