use std::cmp::Ordering;
use std::collections::{hash_map, HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::{db::FileRecord, miko::{Miko, ShrineDestroyer}};
use exemplar::Model;
use rusqlite::{fallible_iterator::IteratorExt, Connection, params};
use anyhow::{Result, anyhow};
use uuid::serde::simple;

type SQMiko = Miko<(Connection, Connection)>;

#[derive(Debug, Clone)]
pub struct DirTreeNode {
    pub dirpath: String,
    pub files: HashMap<String, FileRecord>,
    pub subdirs: HashMap<String, DirTreeNode>,
}

impl DirTreeNode {
    pub fn new(dirpath: &str) -> Self {
        DirTreeNode { dirpath:dirpath.to_string(), files: HashMap::new(), subdirs: HashMap::new() }
    }
}

#[derive(Debug, Clone)]
pub struct FacadeFS {
    miko: SQMiko,
}

const GET_DIRS_IN_DIR_SQL: &str = "
            select distinct 
	            substr(F.file_vfs_path, 0, instr(substr(F.file_vfs_path, length(?1)), '/')+length(?1)) as foldername
            from Files F where F.file_vfs_path like ?1 and F.file_vfs_path != trim(?1, '%');";

const GET_FILES_IN_DIR_SQL: &str = "select * from Files F where F.file_vfs_path = ?1;";
const GET_FILES_IN_SUBDIRS_SQL: &str = "select * from Files F where F.file_vfs_path like ?1 order by length(F.file_vfs_path);";

impl FacadeFS {
    pub fn new(miko: SQMiko) -> Self {
        Self { miko }
    }

    pub fn get_directories_at(&self, dirpath: &str) -> Result<Vec<String>> {
        let dirpath_string = dirpath.to_string();
        let ret: Vec<String> = self.miko.send_mutating_messenger(move |(_, conn)| {
            let mut stmt = conn.prepare_cached(GET_DIRS_IN_DIR_SQL)?;  

            let ret = stmt.query_map([dirpath_string+"%"], |r| {
                Ok(r.get("foldername")?)
            })?.filter(|t| t.is_ok()).map(|t| t.expect("filter didn't work")).collect();

            Ok(ret)
        })?;
        return Ok(ret);
    }

    pub fn get_files_at(&self, dirpath: &str) -> Result<Vec<FileRecord>> {
        let dirpath_string = dirpath.to_string();
        let ret: Vec<FileRecord> = self.miko.send_mutating_messenger(move |(_, conn)| {
            let mut stmt = conn.prepare_cached(GET_FILES_IN_DIR_SQL)?;  

            let ret = stmt.query_map([dirpath_string], |r| {
                FileRecord::from_row(r)
            })?.filter(|t| t.is_ok()).map(|t| t.expect("filter didn't work")).collect();

            Ok(ret)
        })?;
        return Ok(ret);
    }

    pub fn get_dir_tree_at(&self, dirpath: &str) -> Result<DirTreeNode> {
        let mut node = DirTreeNode::new(dirpath);
        for file in self.get_files_at(dirpath)? {
            node.files.insert(file.file_name.clone(), file);
        };
        let folderpaths = self.get_directories_at(dirpath)?;
        for folderpath in folderpaths {
            let childnode = self.get_dir_tree_at(folderpath.as_str())?;
            let filepart = Path::new(folderpath.as_str()).file_name().unwrap().to_str().unwrap().to_string();
            node.subdirs.insert(filepart, childnode);
        }
        Ok(node)
    }

}

#[cfg(test)]
mod facadefs_tests {

    static TESTING_VALUES: &'static str = include_str!("./testing_data/sql/testing_values.sql");
    static INIT_DB_STR: &'static str = include_str!("./db/init_db.sql");

    use proptest::strategy::W;

    use super::*;

    fn init(dbname: &str) -> Result<(FacadeFS, ShrineDestroyer)> {
        //let conn = init_db("./tmp/test_generated_db.sqlite")?;
        let (miko, destroyer): (Miko<(Connection, Connection)>, ShrineDestroyer) =
            Miko::construct_connection_shrine(
                format!("file:{}?mode=memory&cache=shared", dbname).into(),
                &(INIT_DB_STR.to_string() + TESTING_VALUES),
            )?;
        let ffs = FacadeFS::new(miko);
        return Ok((ffs, destroyer));
    }

    #[test]
    fn tests_gets_root() -> Result<()> {
        let (ffs, _d) = init("gets_root")?;
        let rootlist = ffs.get_directories_at("")?;
        assert!(rootlist.contains(&"alpha/".to_string()));
        assert!(rootlist.contains(&"beta/".to_string()));
        assert!(rootlist.contains(&"mastersystem/".to_string()));
        assert!(rootlist.contains(&"pico8/".to_string()));
        Ok(())
    }

    #[test]
    fn tests_get_lower_folder() -> Result<()> {
        let (ffs, _d) = init("gets_lower_folder")?;
        let dirlist = ffs.get_directories_at("beta/")?;
        assert!(dirlist.contains(&"beta/gamma/".to_string()));
        Ok(())
    }

    #[test]
    fn tests_gets_files_from_dir() -> Result<()> {
        let (ffs, _d) = init("gets_file")?;
        let filelist = ffs.get_files_at("alpha/only_one_file/")?;
        assert!(filelist.get(0).expect("Query should return 1 item").file_name == "something.png");
        let filelist_2 = ffs.get_files_at("pico8/")?;
        assert!(filelist_2.len() == 5);
        Ok(())
    }


    #[test]
    fn tests_get_tree() -> Result<()> {
        let (ffs, _d) = init("gets_tree")?;
        let tree = ffs.get_dir_tree_at("pico8/")?;
        println!("The dir tree is: {:?}", tree);
        println!("The dir tree length is: {:?}", tree.files.len());
        assert!(tree.dirpath.as_str() == "pico8/");
        assert!(tree.files.len() == 5);
        assert!(tree.subdirs.get("celeste").unwrap().files.len() == 2);
        Ok(())
    }

}
