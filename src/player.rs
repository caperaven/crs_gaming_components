use std::collections::HashMap;

/// Player is a structure for basic player data
/// This includes the player id, display title and a properties hashmap
/// For flexiblity purposes the hashmap is <String, String> allowing you to store any basic value
pub struct Player {
    pub id          : String,
    pub title       : String,
    pub properties  : HashMap<String, String>
}

impl Player {
    pub fn new(id: String, title: String) -> Player {
        Player {
            id,
            title,
            properties: HashMap::new()
        }
    }

    pub fn get_property(&self, key: String) -> Option<String> {
        let result = self.properties.get(&key);
        match result {
            None => return None,
            Some(value) => Some(value.to_string())
        }
    }

    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player;

    #[test]
    fn player_tests() {
        let mut player = Player::new(String::from("player 1"), String::from("John Doe"));
        assert_eq!(player.id, String::from("player 1"));
        assert_eq!(player.title, String::from("John Doe"));

        player.set_property("age".to_string(), 20.to_string());
        assert_eq!(player.get_property("age".to_string()).unwrap(), 20.to_string());
        assert_eq!(player.get_property("none".to_string()), None);
    }
}
