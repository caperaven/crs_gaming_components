pub trait GameBoard<T> {
    fn size(&self) -> usize;
    fn get(&self, row: usize, col: usize) -> T;
    fn set(&mut self, row: usize, col: usize, value: T);
    fn reset(&mut self, value: T);
}

pub struct Board<T, const SIZE: usize> {
    data: [[T;SIZE];SIZE]
}

impl<T: Copy, const SIZE: usize> Board<T, SIZE> {
    pub fn new(default_value: T) -> Board<T, SIZE> {
        Board {
            data: [[default_value; SIZE]; SIZE]
        }
    }
}

impl<T: Copy, const SIZE: usize> GameBoard<T> for Board<T, SIZE> {
    fn size(&self) -> usize {
        SIZE
    }

    fn get(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row][col] = value;
    }

    fn reset(&mut self, value: T) {
        for row in 0..SIZE {
            for col in 0..SIZE {
                self.data[row][col] = value;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::boards::{Board, GameBoard};

    #[test]
    fn create_board_test() {
        let mut board = Board::<i8,3>::new(-1_i8);
        board.set(0, 0, 0);
        board.set(0, 1, 1);
        assert_eq!(board.size(), 3);
        assert_eq!(board.get(0, 0), 0);
        assert_eq!(board.get(0, 1), 1);
        assert_eq!(board.get(0, 2), -1);
    }
}