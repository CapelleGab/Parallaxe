use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One distinct type per entity, so a `TagId` can never be passed where an `AssetId` is expected.
macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// UUID v7 is time-ordered, which keeps SQLite indexes compact.
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Uuid::parse_str(s).map(Self)
            }
        }
    };
}

id_type!(LibraryId);
id_type!(RootId);
id_type!(AssetId);
id_type!(TagId);
id_type!(CollectionId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_text() {
        let id = TagId::new();
        let parsed: TagId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn rejects_invalid_text() {
        assert!("not-a-uuid".parse::<AssetId>().is_err());
    }

    #[test]
    fn new_ids_are_ordered_by_creation() {
        let ids: Vec<LibraryId> = (0..100_000).map(|_| LibraryId::new()).collect();
        assert!(ids.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn serializes_as_plain_string() {
        let id = RootId::new();
        let json = serde_json::to_value(id).unwrap();
        assert_eq!(json, serde_json::Value::String(id.to_string()));
    }
}
