/// This trait helps you manage scores by adding and removing score values from a structure
pub trait Score {
    fn add_score(&mut self, value: i32) -> i32;
    fn remove_score(&mut self, value: i32) -> i32;
}

/// This trait helps you manage max scores.
/// When you track scores but there is a ceiling you want to stop once that ceiling is reached.
/// This trait allows you to manage that and to check if you have reached the ceiling
pub trait MaxScore {
    fn set_max_score(&mut self, value: i32);
    fn get_max_score(&mut self) -> i32;
    fn at_max(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::score::Score;
    use crate::score::MaxScore;

    struct Player {
        score: i32
    }

    impl Score for Player {
        fn add_score(&mut self, value: i32) -> i32 {
            self.score += value;
            self.score
        }

        fn remove_score(&mut self, value: i32) -> i32 {
            self.score -= value;
            self.score
        }
    }

    struct Team {
        max_score   : i32,
        score       : i32
    }

    impl Score for Team {
        fn add_score(&mut self, value: i32) -> i32 {
            self.score += value;

            if self.score > self.max_score {
                self.score = self.max_score;
            }

            self.score
        }

        fn remove_score(&mut self, value: i32) -> i32 {
            self.score -= value;

            if self.score < 0 {
                self.score = 0;
            }

            self.score
        }
    }

    impl MaxScore for Team {
        fn set_max_score(&mut self, value: i32) {
            self.max_score = value;
        }

        fn get_max_score(&mut self) -> i32 {
            self.max_score
        }

        fn at_max(&self) -> bool {
            return self.score >= self.max_score;
        }
    }

    #[test]
    fn score_management_test() {
        let mut player = Player {score: 0};
        assert_eq!(player.add_score(10), 10);
        assert_eq!(player.remove_score(3), 7);
    }

    #[test]
    fn max_score_tests() {
        let mut team1 = Team { max_score: 100, score: 0 };
        let mut team2 = Team { max_score: 100, score: 0 };

        // set and check max score
        team1.set_max_score(200);
        team2.set_max_score(200);
        assert_eq!(team1.max_score, 200);
        assert_eq!(team2.max_score, 200);

        // add score and check at max for false
        team1.add_score(100);
        assert_eq!(team1.score, 100);
        assert_eq!(team2.score, 0);
        assert_eq!(team1.at_max(), false);
        assert_eq!(team2.at_max(), false);

        // add score and check at max for false
        team1.add_score(150);
        assert_eq!(team1.score, 200);
        assert_eq!(team2.score, 0);
        assert_eq!(team1.at_max(), true);
        assert_eq!(team2.at_max(), false);
    }
}
