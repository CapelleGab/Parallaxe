use std::path::PathBuf;

use crate::id::{CollectionId, LibraryId, RootId, TagId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Library {
    pub id: LibraryId,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootStatus {
    Available,
    Offline,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Root {
    pub id: RootId,
    pub library_id: LibraryId,
    pub path: PathBuf,
    pub status: RootStatus,
    pub last_seen_at: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: TagId,
    pub library_id: LibraryId,
    pub name: String,
    pub color: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    pub id: CollectionId,
    pub library_id: LibraryId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
