pub mod attributes;
pub mod cards;
pub mod inventory;
pub mod matrix_board;
pub mod player;
pub mod score;
pub mod turn_base;
pub mod boards;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
