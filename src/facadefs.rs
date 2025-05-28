use crate::{db::FileRecord, miko::Miko};
use rusqlite::{fallible_iterator::IteratorExt, Connection};

type SQMiko = Miko<(Connection, Connection)>;

pub enum DirItem {
    File(FileRecord),
    Dir(String)
}

#[derive(Debug, Clone)]
pub struct FacadeFS {
    miko: SQMiko,
}

impl FacadeFS {
    pub fn new(miko: SQMiko) -> Self {
        Self { miko }
    }

    pub fn get_root() -> DirItem {
        return DirItem::Dir("/".into())
    }
}
