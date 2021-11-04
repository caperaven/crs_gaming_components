pub struct Turns<'a> {
    current_player_index: usize,
    pub player_ids: Vec<&'a str>
}

impl Turns<'_> {
    pub fn new(player_ids: Vec<&str>) -> Turns {
        Turns {
            current_player_index: 0,
            player_ids
        }
    }

    pub fn next(&mut self) -> &str {
        let mut index = self.current_player_index + 1;

        if index > self.player_ids.len() - 1 {
            index = 0;
        }

        self.current_player_index = index;
        return self.current_player_id();
    }

    pub fn current_player_id(&self) -> &str {
        self.player_ids[self.current_player_index]
    }
}

#[cfg(test)]
mod tests {
    use crate::turn_base::Turns;

    #[test]
    fn turns_test() {
        let mut players = Vec::new();

        let player1_id = "player 1";
        let player2_id = "player 2";
        let player3_id = "player 3";

        players.push(player1_id);
        players.push(player2_id);
        players.push(player3_id);

        let mut turns = Turns::new(players);
        assert_eq!(turns.current_player_id(), player1_id);

        turns.next();
        assert_eq!(turns.current_player_id(), player2_id);
        turns.next();
        assert_eq!(turns.current_player_id(), player3_id);
        turns.next();
        assert_eq!(turns.current_player_id(), player1_id);
    }
}
