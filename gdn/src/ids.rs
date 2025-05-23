use std::{fmt, str::FromStr};

use jiff::{Timestamp, Zoned, tz::TimeZone};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A timestamp- and randomness-based id.
///
/// The id consists of 8 bytes. The first 5 bytes are an epoch timestamp in
/// seconds, while the remaining 3 bytes are a random number, meant to avoid
/// collisions when two ids are created during the same second, possibly even on
/// different devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TimestampId(u64);

impl TimestampId {
    fn new() -> Self {
        let secs: u64 = Timestamp::now()
            .as_second()
            .try_into()
            .expect("timestamp out of range");
        assert!(secs < 0x000000FF_FFFFFFFF_u64, "timestamp out of range");
        let random = rand::random::<u64>() & 0x00000000_00FFFFFF_u64;
        Self(secs << (3 * 8) | random)
    }

    fn timestamp(self) -> Timestamp {
        let secs = self.0 >> (3 * 8);
        Timestamp::from_second(secs as i64).expect("timestamp out of range")
    }
}

impl fmt::Display for TimestampId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

impl FromStr for TimestampId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 16 {
            return Err(());
        }

        if s.chars().any(|c| c.is_lowercase()) {
            return Err(());
        }

        let n = u64::from_str_radix(s, 16).map_err(|_| ())?;

        Ok(Self(n))
    }
}

/// The id for a single note.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoteId(TimestampId);

impl NoteId {
    #[allow(clippy::new_without_default)] // Because side-effects
    pub fn new() -> Self {
        Self(TimestampId::new())
    }

    pub fn timestamp(self) -> Timestamp {
        self.0.timestamp()
    }

    pub fn time_utc(self) -> Zoned {
        self.timestamp().to_zoned(TimeZone::UTC)
    }
}

impl fmt::Debug for NoteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NoteId({self})")
    }
}

impl fmt::Display for NoteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "n{}", self.0)
    }
}

impl FromStr for NoteId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("n").ok_or(())?;
        Ok(Self(s.parse()?))
    }
}

impl Serialize for NoteId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NoteId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&'de str as Deserialize<'de>>::deserialize(deserializer)?
            .parse()
            .map_err(|()| serde::de::Error::custom("invalid note id"))
    }
}

/// The id for a note repository.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RepoId(TimestampId);

impl RepoId {
    #[allow(clippy::new_without_default)] // Because side-effects
    pub fn new() -> Self {
        Self(TimestampId::new())
    }
}

impl fmt::Debug for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RepoId({self})")
    }
}

impl fmt::Display for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

impl FromStr for RepoId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("r").ok_or(())?;
        Ok(Self(s.parse()?))
    }
}

impl Serialize for RepoId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RepoId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&'de str as Deserialize<'de>>::deserialize(deserializer)?
            .parse()
            .map_err(|()| serde::de::Error::custom("invalid repo id"))
    }
}
