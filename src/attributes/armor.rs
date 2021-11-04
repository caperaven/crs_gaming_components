pub trait Armor<T> {
    fn set_armor(&mut self, value: T);
    fn get_armor(&self) -> T;
}

#[cfg(test)]
pub mod test {
    use crate::attributes::Armor;

    struct Player {
        armor: i32
    }

    impl Armor<i32> for Player {
        fn set_armor(&mut self, value: i32) {
            self.armor = value;
        }

        fn get_armor(&self) -> i32 {
            self.armor
        }
    }

    #[test]
    fn armor_change_test() {
        let mut player = Player { armor: 10 };
        player.set_armor(100);
        assert_eq!(player.get_armor(), 100);
    }
}