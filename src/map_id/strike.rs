pub use self::{eod::*, ibs::*, ls1::*, soto::*, wintersday::*};

/// Wintersday former Strike Missions.
pub mod wintersday {
    pub use crate::map_id::raid::SECRET_LAIR_OF_THE_SNOWMEN;
}

/// Icebrood Saga former Strike Missions.
pub mod ibs {
    pub use crate::map_id::raid::{
        BONESKINNER, COLD_WAR, FORGING_STEEL, FRAENIR_OF_JORMAG, SHIVERPEAKS_PASS, VOICE_AND_CLAW,
        WHISPER_OF_JORMAG,
    };
}

/// End of Dragons former Strike Missions.
pub mod eod {
    pub use crate::map_id::raid::{
        AETHERBLADE_HIDEOUT, HARVEST_TEMPLE, KAINENG_OVERLOOK, XUNLAI_JADE_JUNKYARD,
    };
}

/// Living World Season 1 (rework) former Strike Missions.
pub mod ls1 {
    pub use crate::map_id::raid::OLD_LIONS_COURT;
}

/// Secrets of the Obscure former Strike Missions.
pub mod soto {
    pub use crate::map_id::raid::{COSMIC_OBSERVATORY, TEMPLE_OF_FEBE};
}
