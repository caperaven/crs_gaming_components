#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Rank {
    Number,
    Jack,
    Queen,
    King,
    Ace,
    JokerRed,
    JokerBlack
}