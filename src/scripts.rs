use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Script {
    pub code: String,
}

pub fn load_scripts() -> Vec<Script> {
    vec![
        Script { code: "print('Hello, World!')".to_string() },
        Script { code: "game.Players.LocalPlayer:Kick('You have been kicked!')".to_string() },
    ]
}