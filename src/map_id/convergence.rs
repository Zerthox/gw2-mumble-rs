pub use self::{jw::*, soto::*, voe::*};

/// Secrets of the Obscure Convergences.
pub mod soto {
    /// Outer Nayos (Public).
    pub const OUTER_NAYOS_PUBLIC: u32 = 1523;

    /// Outer Nayos (Private Squad).
    pub const OUTER_NAYOS_PRIVATE: u32 = 1527;
}

/// Janthir Wilds Convergences.
pub mod jw {
    /// Mount Balrior (Public).
    pub const MOUNT_BALRIOR_PUBLIC: u32 = 1571;

    /// Mount Balrior (Private Squad).
    pub const MOUNT_BALRIOR_PRIVATE: u32 = 1562;
}

/// Visions of Eternity Convergences.
pub mod voe {
    /// Nexus of Eternity (Public).
    pub const NEXUS_OF_ETERNITY_PUBLIC: u32 = 1627;

    /// Nexus of Eternity (Private Squad).
    pub const NEXUS_OF_ETERNITY_PRIVATE: u32 = 1629;
}
