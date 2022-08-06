use std::fmt::Display;

use color_eyre::Report;
use uuid::Uuid;

use crate::packet::{CostumePacket, GamePacket, PlayerPacket};

// region: Player
#[derive(Debug)]
pub struct Player {
    pub id: Uuid,
    pub name: String,

    pub loaded: bool,
    pub costume: Option<Costume>,
    pub scenario: Option<u8>,
    pub is_2d: bool,

    pub last_pos: Option<PlayerPacket>,
    pub last_game: Option<GamePacket>,
}

impl Player {
    #[inline]
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,

            loaded: false,
            costume: None,
            scenario: None,
            is_2d: false,

            last_pos: None,
            last_game: None,
        }
    }

    #[inline]
    pub fn stage(&self) -> Option<&str> {
        self.last_game
            .as_ref()
            .and_then(|x| x.stage.try_as_str().ok())
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.name, self.id)
    }
}
// endregion

// region: Costume
#[derive(Debug, Clone)]
pub struct Costume {
    pub body: String,
    pub cap: String,
}

impl Default for Costume {
    #[inline]
    fn default() -> Self {
        Self {
            body: "Mario".into(),
            cap: "Mario".into(),
        }
    }
}

impl TryFrom<CostumePacket> for Costume {
    type Error = Report;

    #[inline]
    fn try_from(packet: CostumePacket) -> Result<Self, Self::Error> {
        let body = packet.body.try_into()?;
        let cap = packet.body.try_into()?;

        Ok(Self { body, cap })
    }
}

impl TryFrom<Costume> for CostumePacket {
    type Error = Report;

    #[inline]
    fn try_from(costume: Costume) -> Result<Self, Self::Error> {
        let body = costume.body.parse()?;
        let cap = costume.body.parse()?;

        Ok(Self { body, cap })
    }
}
// endregion
