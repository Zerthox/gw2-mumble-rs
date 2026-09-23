pub use self::{core::*, eod::*, hot::*, ibs::*, jw::*, pof::*, soto::*, voe::*};

/// Lion's Arch Aerodrome (Raid lobby).
pub const AERODROME: u32 = 1155;

/// Special Forces Training Area (Golem).
pub const TRAINING_AREA: u32 = 1154;

/// Core game Raids.
pub mod core {
    /// Secret Lair of the Snowmen (Freezie).
    pub const SECRET_LAIR_OF_THE_SNOWMEN: u32 = 1306;

    /// Old Lion's Court (Watchknight Triumvirate).
    pub const OLD_LIONS_COURT: u32 = 1485;
}

/// Heart of Thorns Raids.
pub mod hot {
    /// Spirit Vale (Wing 1).
    pub const SPIRIT_VALE: u32 = 1062;

    /// Salvation Pass (Wing 2).
    pub const SALVATION_PASS: u32 = 1149;

    /// Stronghold of the Faithful (Wing 3).
    pub const STRONGHOLD_OF_THE_FAITHFUL: u32 = 1156;

    /// Bastion of the Penitent (Wing 4).
    pub const BASTION_OF_THE_PENITENT: u32 = 1188;
}

/// Path of Fire Raids.
pub mod pof {
    /// Hall of Chains (Wing 5).
    pub const HALL_OF_CHAINS: u32 = 1264;

    /// Mythwright Gambit (Wing 6).
    pub const MYTHWRIGHT_GAMBIT: u32 = 1303;

    /// The Key of Ahdashim (Wing 7).
    pub const KEY_OF_AHDASHIM: u32 = 1323;
}

/// The Icebrood Saga Raids.
pub mod ibs {
    /// Shiverpeaks Pass (Icebrood Construct).
    pub const SHIVERPEAKS_PASS: u32 = 1332;

    /// Boneskinner.
    pub const BONESKINNER: u32 = 1339;

    /// Fraenir of Jormag.
    pub const FRAENIR_OF_JORMAG: u32 = 1341;

    /// Voice of the Fallen and Claw of the Fallen.
    pub const VOICE_AND_CLAW: u32 = 1346;

    /// Whisper of Jormag.
    pub const WHISPER_OF_JORMAG: u32 = 1359;

    /// Forging Steel (Ancient Forgeman).
    pub const FORGING_STEEL: u32 = 1368;

    /// Cold War (Minister of Morale).
    pub const COLD_WAR: u32 = 1374;
}

/// End of Dragons Strike Missions.
pub mod eod {
    /// Aetherblade Hideout (Mai Trin).
    pub const AETHERBLADE_HIDEOUT: u32 = 1432;

    /// Xunlai Jade Junkyard (Ankka).
    pub const XUNLAI_JADE_JUNKYARD: u32 = 1450;

    /// Kaineng Overlook (Minister Li).
    pub const KAINENG_OVERLOOK: u32 = 1451;

    /// Harvest Temple (The Dragonvoid).
    pub const HARVEST_TEMPLE: u32 = 1437;
}

/// Secrets of the Obscure Raids.
pub mod soto {
    /// Cosmic Observatory (Dagda).
    pub const COSMIC_OBSERVATORY: u32 = 1515;

    /// Temple of Febe (Cerus).
    pub const TEMPLE_OF_FEBE: u32 = 1520;
}

/// Janthir Wilds Raids.
pub mod jw {
    /// Mount Balrior (Wing 8).
    pub const MOUNT_BALRIOR: u32 = 1564;
}

/// Visions of Eternity Raids.
pub mod voe {
    /// Guardian's Glade (Kela).
    pub const GUARDIANS_GLADE: u32 = 1609;

    /// Nexus of Eternity (Vloxx).
    pub const NEXUS_OF_ETERNITY: u32 = 1638;
}
