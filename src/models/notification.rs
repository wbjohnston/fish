use super::game::GameId;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub game_id: GameId,
}
