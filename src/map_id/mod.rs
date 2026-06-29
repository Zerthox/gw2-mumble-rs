//! Known map ids.

/// Raid Wings & Raid Encounters.
pub mod raid;

/// Fractals of the Mists.
pub mod fractal;

/// Former Strike Missions.
///
/// Renamed to Raid Encounters.
#[deprecated = "use via raid"]
pub mod strike;

/// Structured PvP.
pub mod pvp;

/// World versus World.
pub mod wvw;

/// Activities.
pub mod activity;

/// Player Hubs.
pub mod hub {
    pub use super::{fractal::MISTLOCK_OBSERVATORY, pvp::PVP_LOBBY, raid::AERODROME};

    /// Lions's Arch.
    pub const LIONS_ARCH: u32 = 50;

    /// Mistlock Sanctuary.
    pub const MISTLOCK_SANCTUARY: u32 = 1206;

    /// Eye of the North (IBS).
    pub const EYE_OF_THE_NORTH: u32 = 1370;

    /// Arborstone (EOD).
    pub const ARBORSTONE: u32 = 1428;

    /// Wizard's Tower (SOTO).
    pub const WIZARDS_TOWER: u32 = 1509;
}
