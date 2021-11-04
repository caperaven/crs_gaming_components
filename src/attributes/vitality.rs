/// Vitality trait
///
/// This trait allows for managing vitality
pub trait Vitality<T> {
    fn is_alive(&self) -> bool;
    fn set_vitality(&mut self, value: T);
    fn get_vitality(&self) -> T;
}

/// Vitality Modifier
///
/// This trait allows for increasing and decreasing vitality
pub trait VitalityModifier<T: Vitality<T>> {
    fn inc_vitality(&self, value: T);
    fn dec_vitality(&self, value: T);
}

/// Vitality Modifier Timed
///
/// This trait allows managing vitality over time
pub trait VitalityModifierTimed<T: Vitality<T> + VitalityModifier<T>> {
    fn inc_vitality_over_time(&self, value: T, every: f32, duration: f32);
}

#[cfg(test)]
pub mod test {
    use crate::attributes::Vitality;

    struct Player {
        vitality: i32
    }

    impl Player {
        pub fn new() -> Player {
            Player {
                vitality: 100
            }
        }
    }

    impl Vitality<i32> for Player {
        fn is_alive(&self) -> bool {
            self.vitality > 0
        }

        fn set_vitality(&mut self, value: i32) {
            self.vitality = value;
        }

        fn get_vitality(&self) -> i32 {
            self.vitality
        }
    }

    #[test]
    fn vitaility_is_alive_test() {
        let mut player = Player::new();
        assert_eq!(player.get_vitality(), 100);
        assert_eq!(player.is_alive(), true);

        player.set_vitality(0);
        assert_eq!(player.is_alive(), false);
        assert_eq!(player.get_vitality(), 0);
    }
}