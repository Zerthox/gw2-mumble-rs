/// Information about player identity.
///
/// Parsed as JSON from the `identity` field in [`LinkedMem`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct Identity {
    /// Character name.
    pub name: String,

    /// Character profession.
    pub profession: Profession,

    /// Equipped 3rd specialization.
    ///
    /// `0` if no specialization is present.
    /// See [API:2/specializations](https://wiki.guildwars2.com/wiki/API:2/specializations) for details.
    pub spec: u32,

    /// Character race.
    pub race: Race,

    /// Current map id.
    ///
    /// See [API:2/maps](https://wiki.guildwars2.com/wiki/API:2/maps).
    pub map_id: u32,

    /// Shard id from [`Context`].
    ///
    /// Formerly character homeworld per [API:2/worlds](https://wiki.guildwars2.com/wiki/API:2/worlds).
    /// Not usable since the switch to the megaserver system.
    pub world_id: u32,

    /// Team color.
    ///
    /// See [API:2/colors](https://wiki.guildwars2.com/wiki/API:2/colors).
    pub team_color_id: u32,

    /// Whether the character has a commander tag active.
    pub commander: bool,

    /// Vertical field of view.
    pub fov: f32,

    /// Current user UI scaling.
    #[cfg_attr(feature = "serde", serde(rename = "uisz"))]
    pub ui_scale: UIScaling,
}

/// Character profession.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::Display,
    strum::AsRefStr,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
pub enum Profession {
    Guardian = 1,
    Warrior = 2,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
    Revenant = 9,
    Monster = 10,
}

/// Character race.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::Display,
    strum::AsRefStr,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
pub enum Race {
    Asura = 0,
    Charr = 1,
    Human = 2,
    Norn = 3,
    Sylvari = 4,
}

/// User UI scaling.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::Display,
    strum::AsRefStr,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
pub enum UIScaling {
    Small = 0,
    Normal = 1,
    Large = 2,
    Larger = 3,
}
