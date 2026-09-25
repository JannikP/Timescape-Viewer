use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// Enumeration of physical units. Besides the special cases for unit-less values ([Unit::None]) and
/// custom units ([Unit::Custom]) this enum contains variants for many units defined by the
/// International System of Units (SI). Using the provided variants for well-known units improves
/// the user experience, as tooltips can be localized and unit specific conversions might be
/// possible.
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    /// No physical unit specified.
    #[default]
    None,

    /// Not a well known unit.
    Custom(String),

    //-------------------------------------------------------------------------
    // [SI Base Units](https://en.wikipedia.org/wiki/International_System_of_Units)
    //-------------------------------------------------------------------------

    /// The second (symbol: s) is the unit of time in the International System
    /// of Units (SI).
    Second,

    /// The metre (or meter in US spelling; symbol: m) is the base unit of
    /// length in the International System of Units (SI).
    Metre,

    /// The kilogram (also spelled kilogramme) is the base unit of mass in the
    /// International System of Units (SI), equal to one thousand grams.
    Kilogram,

    /// The ampere (symbol: A) is the unit of electric current in the
    /// International System of Units (SI).
    Ampere,

    /// The kelvin (symbol: K) is the base unit for temperature in the
    /// International System of Units (SI). The Kelvin scale is an absolute
    /// temperature scale that starts at the lowest possible temperature
    /// (absolute zero), taken to be 0 K.
    Kelvin,

    /// The mole (symbol mol) is a unit of measurement, the base unit in the
    /// International System of Units (SI) for amount of substance.
    Mole,

    /// Candela (symbol: cd) is the SI unit of luminous intensity. It measures
    /// the luminous power per unit solid angle emitted in a particular
    /// direction.
    Candela,

}

impl Unit {
    pub fn description(&self) -> Option<Cow<'_, str>> {
        // TODO: localize the strings using `rust-i18n`.
        match self {
            Unit::None | Unit::Custom(_) => None,
            Unit::Second => Some(t!("unit.second.description")),
            Unit::Metre => Some(t!("unit.metre.description")),
            Unit::Kilogram => Some(t!("unit.kilogram.description")),
            Unit::Ampere => Some(t!("unit.ampere.description")),
            Unit::Kelvin => Some(t!("unit.kelvin.description")),
            Unit::Mole => Some(t!("unit.mole.description")),
            Unit::Candela => Some(t!("unit.candela.description")),
        }
    }

    pub fn name<'a>(&'a self) -> Option<Cow<'a, str>> {
        // TODO: localize the strings using `rust-i18n`.
        match self {
            Unit::None => None,
            Unit::Custom(value) => Some(value.into()),
            Unit::Second => Some(t!("unit.second.name")),
            Unit::Metre => Some(t!("unit.metre.name")),
            Unit::Kilogram => Some(t!("unit.kilogram.name")),
            Unit::Ampere => Some(t!("unit.ampere.name")),
            Unit::Kelvin => Some(t!("unit.kelvin.name")),
            Unit::Mole => Some(t!("unit.mole.name")),
            Unit::Candela => Some(t!("unit.candela.name")),
        }
    }

    pub fn symbol<'a>(&'a self) -> Option<&'a str> {
        match self {
            Unit::None => None,
            Unit::Custom(value) => Some(value.as_str()),
            Unit::Second => Some("s"),
            Unit::Metre => Some("m"),
            Unit::Kilogram => Some("kg"),
            Unit::Ampere => Some("A"),
            Unit::Kelvin => Some("K"),
            Unit::Mole => Some("mol"),
            Unit::Candela => Some("cd"),
        }
    }
}

// TODO: impl From<&str>
// TODO: impl From<String>
// TODO: impl sqlx::Type

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol().unwrap_or(""))
    }
}
