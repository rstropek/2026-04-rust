mod tic_tac_toe {
    use std::fmt::Display;
    use std::str::FromStr;

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum Player {
        X,
        O,
    }

    #[derive(Debug)]
    pub struct Coordinate {
        x: usize,
        y: usize,
    }

    #[derive(Debug)]
    pub enum CoordinateParseError {
        /// Input longer than 2 chars (=2 bytes)
        InputTooLong,

        /// Input shorter than 2 chars (=2 bytes)
        InputTooShort,

        /// First char is not A-C
        InvalidRow,

        /// Second char is not 1-3
        InvalidColumn,
    }

    impl FromStr for Coordinate {
        type Err = CoordinateParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.trim();
            if s.len() > 2 {
                return Err(CoordinateParseError::InputTooLong);
            }
            if s.len() < 2 {
                return Err(CoordinateParseError::InputTooShort);
            }

            let bytes = s.as_bytes();
            let x = match bytes[0] {
                b'A' | b'a' => 0,
                b'B' | b'b' => 1,
                b'C' | b'c' => 2,
                _ => return Err(CoordinateParseError::InvalidRow),
            };
            let y = match bytes[1] {
                b'1' => 0,
                b'2' => 1,
                b'3' => 2,
                _ => return Err(CoordinateParseError::InvalidColumn),
            };

            Ok(Coordinate { x, y })
        }
    }

    impl Display for CoordinateParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                CoordinateParseError::InputTooLong => {
                    write!(f, "Input is too long (expected 2 characters, e.g. A1)")
                }
                CoordinateParseError::InputTooShort => {
                    write!(f, "Input is too short (expected 2 characters, e.g. A1)")
                }
                CoordinateParseError::InvalidRow => {
                    write!(f, "Invalid row (expected A, B, or C)")
                }
                CoordinateParseError::InvalidColumn => {
                    write!(f, "Invalid column (expected 1, 2, or 3)")
                }
            }
        }
    }

    #[derive(Debug)]
    pub enum GameResult {
        Win(Player),
        Draw,
        Ongoing,
    }

    #[derive(Debug)]
    pub enum PlayTurnError {
        InvalidCoordinate,
        CellOccupied,
        AlreadyWon,
    }

    impl Display for PlayTurnError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                PlayTurnError::InvalidCoordinate => write!(f, "Invalid coordinate"),
                PlayTurnError::CellOccupied => write!(f, "Cell is already occupied"),
                PlayTurnError::AlreadyWon => write!(f, "Game already has a winner"),
            }
        }
    }

    /// Represents a 3x3 matrix for the tic tac toe game
    pub struct Matrix {
        board: [[Option<Player>; 3]; 3],
        current_player: Player,
    }

    impl Matrix {
        pub fn play_turn(&mut self, coord: Coordinate) -> Result<GameResult, PlayTurnError> {
            if coord.x >= 3 || coord.y >= 3 {
                return Err(PlayTurnError::InvalidCoordinate);
            }

            if let GameResult::Win(_) = self.get_game_result() {
                return Err(PlayTurnError::AlreadyWon);
            }

            if self.board[coord.x][coord.y].is_some() {
                return Err(PlayTurnError::CellOccupied);
            }

            self.board[coord.x][coord.y] = Some(self.current_player);

            let result = self.get_game_result();

            if matches!(result, GameResult::Ongoing) {
                self.current_player = match self.current_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };
            }

            Ok(result)
        }

        pub fn new() -> Self {
            Matrix {
                board: [[None; 3]; 3],
                current_player: Player::X,
            }
        }

        pub fn get_game_result(&self) -> GameResult {
            // Check rows
            for row in &self.board {
                if let [Some(a), Some(b), Some(c)] = row
                    && a == b
                    && b == c
                {
                    return GameResult::Win(*a);
                }
            }

            // Check columns
            for col in 0..3 {
                if let (Some(a), Some(b), Some(c)) =
                    (self.board[0][col], self.board[1][col], self.board[2][col])
                    && a == b
                    && b == c
                {
                    return GameResult::Win(a);
                }
            }

            // Check main diagonal
            if let (Some(a), Some(b), Some(c)) =
                (self.board[0][0], self.board[1][1], self.board[2][2])
                && a == b
                && b == c
            {
                return GameResult::Win(a);
            }

            // Check anti-diagonal
            if let (Some(a), Some(b), Some(c)) =
                (self.board[0][2], self.board[1][1], self.board[2][0])
                && a == b
                && b == c
            {
                return GameResult::Win(a);
            }

            // Draw if all cells are filled, otherwise ongoing
            let all_filled = self
                .board
                .iter()
                .all(|row| row.iter().all(|cell| cell.is_some()));
            if all_filled {
                GameResult::Draw
            } else {
                GameResult::Ongoing
            }
        }
    }

    impl Default for Matrix {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            writeln!(f, "  1 2 3")?;
            for (i, row) in self.board.iter().enumerate() {
                let label = match i {
                    0 => 'A',
                    1 => 'B',
                    2 => 'C',
                    _ => unreachable!(),
                };
                write!(f, "{}", label)?;
                for cell in row {
                    let symbol = match cell {
                        Some(Player::X) => 'X',
                        Some(Player::O) => 'O',
                        None => '.',
                    };
                    write!(f, " {}", symbol)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::rstest;

        /// Play a sequence of moves on a fresh board and return the result of the final move.
        fn play_sequence(moves: &[&str]) -> (Matrix, Result<GameResult, PlayTurnError>) {
            let mut m = Matrix::new();
            let (last, rest) = moves.split_last().expect("at least one move");
            for mv in rest {
                m.play_turn(mv.parse().unwrap()).unwrap();
            }
            let result = m.play_turn(last.parse().unwrap());
            (m, result)
        }

        // --- Coordinate::from_str ---

        #[rstest]
        #[case::a1("A1", 0, 0)]
        #[case::b2("B2", 1, 1)]
        #[case::c3("C3", 2, 2)]
        #[case::lowercase("b3", 1, 2)]
        #[case::trims_whitespace_and_newline("  A2\n", 0, 1)]
        fn parse_coordinate_valid(#[case] input: &str, #[case] x: usize, #[case] y: usize) {
            let c: Coordinate = input.parse().unwrap();
            assert_eq!(c.x, x);
            assert_eq!(c.y, y);
        }

        #[rstest]
        #[case::too_long("A12", CoordinateParseError::InputTooLong)]
        #[case::too_short("A", CoordinateParseError::InputTooShort)]
        #[case::empty("", CoordinateParseError::InputTooShort)]
        #[case::invalid_row("D1", CoordinateParseError::InvalidRow)]
        #[case::invalid_column("A4", CoordinateParseError::InvalidColumn)]
        fn parse_coordinate_invalid(#[case] input: &str, #[case] expected: CoordinateParseError) {
            let err = input.parse::<Coordinate>().unwrap_err();
            assert!(
                std::mem::discriminant(&err) == std::mem::discriminant(&expected),
                "expected {:?}, got {:?}",
                expected,
                err
            );
        }

        // --- Matrix::new ---

        #[test]
        fn new_matrix_is_empty_with_x_starting() {
            let m = Matrix::new();
            assert_eq!(m.current_player, Player::X);
            for row in &m.board {
                for cell in row {
                    assert!(cell.is_none());
                }
            }
            assert!(matches!(m.get_game_result(), GameResult::Ongoing));
        }

        // --- Matrix::play_turn: positive cases ---

        #[test]
        fn play_turn_places_piece_and_switches_player() {
            let mut m = Matrix::new();
            let result = m.play_turn("A1".parse().unwrap()).unwrap();
            assert!(matches!(result, GameResult::Ongoing));
            assert_eq!(m.board[0][0], Some(Player::X));
            assert_eq!(m.current_player, Player::O);
        }

        // Each case is X's 5th move that completes a line.
        // Moves alternate X, O, X, O, X (X wins).
        #[rstest]
        #[case::row(&["A1", "B1", "A2", "B2", "A3"])]
        #[case::column(&["A1", "A2", "B1", "B2", "C1"])]
        #[case::main_diagonal(&["A1", "A2", "B2", "A3", "C3"])]
        #[case::anti_diagonal(&["A3", "A1", "B2", "A2", "C1"])]
        fn play_turn_win(#[case] moves: &[&str]) {
            let (_, result) = play_sequence(moves);
            assert!(matches!(result.unwrap(), GameResult::Win(Player::X)));
        }

        #[test]
        fn play_turn_draw() {
            // Produces a full board with no winner:
            // X O X
            // X O O
            // O X X
            let moves = ["A1", "A2", "A3", "B2", "B1", "C1", "C2", "B3", "C3"];
            let (_, result) = play_sequence(&moves);
            assert!(matches!(result.unwrap(), GameResult::Draw));
        }

        // --- Matrix::play_turn: error cases ---

        #[rstest]
        #[case::x_out_of_range(Coordinate { x: 3, y: 0 })]
        #[case::y_out_of_range(Coordinate { x: 0, y: 5 })]
        #[case::both_out_of_range(Coordinate { x: 9, y: 9 })]
        fn play_turn_invalid_coordinate(#[case] bad: Coordinate) {
            let mut m = Matrix::new();
            let err = m.play_turn(bad).unwrap_err();
            assert!(matches!(err, PlayTurnError::InvalidCoordinate));
        }

        #[test]
        fn play_turn_cell_occupied() {
            let (_, result) = play_sequence(&["A1", "A1"]);
            assert!(matches!(result.unwrap_err(), PlayTurnError::CellOccupied));
        }

        #[test]
        fn play_turn_already_won() {
            // X wins on move 5 ("A3"), then O tries to play "C3".
            let (_, result) = play_sequence(&["A1", "B1", "A2", "B2", "A3", "C3"]);
            assert!(matches!(result.unwrap_err(), PlayTurnError::AlreadyWon));
        }

        // --- Matrix::get_game_result directly ---

        #[test]
        fn get_game_result_ongoing_on_new() {
            let m = Matrix::new();
            assert!(matches!(m.get_game_result(), GameResult::Ongoing));
        }

        // --- Matrix Display ---

        #[rstest]
        #[case::empty(&[], "  1 2 3\nA . . .\nB . . .\nC . . .\n")]
        #[case::two_pieces(&["A1", "B2"], "  1 2 3\nA X . .\nB . O .\nC . . .\n")]
        fn display_matrix(#[case] moves: &[&str], #[case] expected: &str) {
            let mut m = Matrix::new();
            for mv in moves {
                m.play_turn(mv.parse().unwrap()).unwrap();
            }
            assert_eq!(format!("{}", m), expected);
        }
    }
}

use std::io;

use crate::tic_tac_toe::{GameResult, Matrix};

fn main() {
    // Create instance of Matrix (new)
    let mut matrix = Matrix::new();

    loop {
        // Ask current player for coordinates for current turn
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        // Call play_turn with the coordinates
        let turn_result = match buffer.parse() {
            Ok(coordinates) => matrix.play_turn(coordinates),
            Err(e) => {
                println!("Error parsing coordinates: {}", e);
                continue;
            }
        };

        match turn_result {
            Ok(game_result) => match game_result {
                GameResult::Win(player) => {
                    println!("Player {:?} wins!", player);
                    break;
                }
                GameResult::Draw => {
                    println!("It's a draw!");
                    break;
                }
                GameResult::Ongoing => {
                    // Do nothing, just continue
                }
            },
            Err(e) => {
                println!("Error playing turn: {}", e);
                continue;
            }
        }

        println!("{}", matrix);
    }
}
