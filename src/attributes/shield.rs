pub trait Shield<T> {
    fn set_shield(&mut self, value: T);
    fn get_shield(&self) -> T;
}

#[cfg(test)]
pub mod test {
    use crate::attributes::Shield;

    struct Player {
        shield: i32
    }

    impl Shield<i32> for Player {
        fn set_shield(&mut self, value: i32) {
            self.shield = value;
        }

        fn get_shield(&self) -> i32 {
            self.shield
        }
    }

    #[test]
    fn armor_change_test() {
        let mut player = Player { shield: 0 };
        player.set_shield(100);
        assert_eq!(player.get_shield(), 100);
    }
}