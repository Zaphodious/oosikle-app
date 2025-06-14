use std::cmp::Ordering;
use std::collections::{hash_map, HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::{
    db::FileRecord,
    miko::{Miko, ShrineDestroyer},
};
use anyhow::{anyhow, Result};
use exemplar::Model;
use relative_path::{RelativePath, RelativePathBuf};
use rusqlite::{fallible_iterator::IteratorExt, params, Connection};
use serde::{Deserialize, Serialize};
use uuid::serde::simple;
use fast_glob::glob_match;

type SQMiko = Miko<(Connection, Connection)>;

#[derive(Debug, Clone)]
pub enum CursorIntoItem<'a> {
    Dir(&'a DirTreeNode),
    File(&'a FileRecord),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirTreeNode {
    pub dirpath: String,
    pub files: HashMap<String, FileRecord>,
    pub subdirs: HashMap<String, DirTreeNode>,
}

type FlattenedDir = Vec<(RelativePathBuf, FileRecord)>;

impl DirTreeNode {
    pub fn new(dirpath: &str) -> Self {
        DirTreeNode {
            dirpath: dirpath.to_string(),
            files: HashMap::new(),
            subdirs: HashMap::new(),
        }
    }
    pub fn get_at_path(&self, dirpath: &str) -> Option<CursorIntoItem> {
        //println!("Starting cursor_into with {:?}", dirpath);
        let asrelative = RelativePath::new(dirpath);
        let definitely_paths = asrelative.parent()?;
        let path_end = asrelative
            .file_name()
            .expect("FacadeFS paths should not end in '...'");
        //println!("def paths is {:?}", definitely_paths);
        let mut target_dir = Some(self);
        //println!("target dir is {:?}", target_dir);
        for comp in definitely_paths.components() {
            //println!("Getting for commp {:?}", comp);
            if let Some(target) = target_dir {
                target_dir = target.subdirs.get(comp.as_str());
                //println!("new target is {:?}", target_dir);
            }
        }
        if let Some(final_dir) = target_dir {
            if let Some(the_file) = final_dir.files.get(path_end) {
                Some(CursorIntoItem::File(the_file))
            } else if let Some(the_dir) = final_dir.subdirs.get(path_end) {
                Some(CursorIntoItem::Dir(the_dir))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn search_files_with_names_matching_pattern(
        &self,
        pattern: &str,
    ) -> Result<Vec<FileRecord>> {
        let mut accum = vec![];
        self.do_get_files_with_names_matching_pattern(pattern, &mut accum)?;
        Ok(accum)
    }

    pub fn search_files_with_names_matching_pattern_recursive(
        &self,
        pattern: &str,
    ) -> Result<Vec<FileRecord>> {
        let mut accum = vec![];
        self.do_get_files_with_names_matching_pattern_recursive(pattern, &mut accum)?;
        Ok(accum)
    }

    fn do_get_files_with_names_matching_pattern(
        &self,
        pattern: &str,
        accum: &mut Vec<FileRecord>,
    ) -> Result<()> {
        for (filename, filerecord) in &self.files {
            if glob_match(pattern, &*filename) {
                accum.push(filerecord.clone());
            }
        }
        Ok(())
    }

    fn do_get_files_with_names_matching_pattern_recursive(
        &self,
        pattern: &str,
        accum: &mut Vec<FileRecord>,
    ) -> Result<()> {
        self.do_get_files_with_names_matching_pattern(pattern, accum)?;
        for (_dirname, dir) in &self.subdirs {
            dir.do_get_files_with_names_matching_pattern_recursive(pattern, accum)?;
        }
        Ok(())
    }

    pub fn flatten(&self) -> Result<FlattenedDir> {
        let mut accum = vec![];
        let prefixpath = RelativePath::new("");
        self.do_flatten(prefixpath, &mut accum)?;
        Ok(accum)
    }

    pub fn glob(&self, pattern: &str) -> Result<FlattenedDir> {
        let flatten_res = self
            .flatten()?
            .into_iter()
            .filter(|(filepath, _file)| glob_match(pattern, filepath.as_str()))
            .collect();
        Ok(flatten_res)
    }

    fn do_flatten(&self, prefixpath: &RelativePath, accum: &mut FlattenedDir) -> Result<()> {
        let thispath = prefixpath.join(RelativePath::new(self.dirpath.as_str()));
        for (filename, filerecord) in &self.files {
            let filepath = thispath.join(filename);
            accum.push((filepath, filerecord.clone()));
        }
        for (_dirpath, dir) in &self.subdirs {
            dir.do_flatten(thispath.as_relative_path(), accum)?;
        }
        Ok(())
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
const GET_FILES_IN_SUBDIRS_SQL: &str =
    "select * from Files F where F.file_vfs_path like ?1 order by length(F.file_vfs_path);";
const GET_FILES_MATCHING_GLOB: &str = "select * from Files F where F.file_vfs_path GLOB ?1;";

impl FacadeFS {
    pub fn new(miko: SQMiko) -> Self {
        Self { miko }
    }

    pub fn get_directories_at(&self, dirpath: &str) -> Result<Vec<String>> {
        let dirpath_string = dirpath.to_string();
        let ret: Vec<String> = self.miko.send_mutating_messenger(move |(_, conn)| {
            let mut stmt = conn.prepare_cached(GET_DIRS_IN_DIR_SQL)?;

            let ret = stmt
                .query_map([dirpath_string + "%"], |r| Ok(r.get("foldername")?))?
                .filter(|t| t.is_ok())
                .map(|t| t.expect("filter didn't work"))
                .collect();

            Ok(ret)
        })?;
        return Ok(ret);
    }

    pub fn get_files_at(&self, dirpath: &str) -> Result<Vec<FileRecord>> {
        let dirpath_string = dirpath.to_string();
        let ret: Vec<FileRecord> = self.miko.send_mutating_messenger(move |(_, conn)| {
            let mut stmt = conn.prepare_cached(GET_FILES_IN_DIR_SQL)?;

            let ret = stmt
                .query_map([dirpath_string], |r| FileRecord::from_row(r))?
                .filter(|t| t.is_ok())
                .map(|t| t.expect("filter didn't work"))
                .collect();

            Ok(ret)
        })?;
        return Ok(ret);
    }

    pub fn get_dir_tree_at(&self, dirpath: &str) -> Result<DirTreeNode> {
        let mut node = DirTreeNode::new(dirpath);
        for file in self.get_files_at(dirpath)? {
            node.files.insert(file.file_name.clone(), file);
        }
        let folderpaths = self.get_directories_at(dirpath)?;
        for folderpath in folderpaths {
            let childnode = self.get_dir_tree_at(folderpath.as_str())?;
            let filepart = Path::new(folderpath.as_str())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            node.subdirs.insert(filepart, childnode);
        }
        Ok(node)
    }

    /*
    pub fn glob(&self, startdir: &str, pattern: &str) -> Result<FlattenedDir> {

    } */
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
        assert!(
            filelist
                .get(0)
                .expect("Query should return 1 item")
                .file_name
                == "something.png"
        );
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

    #[test]
    fn tests_cursor_into_works() -> Result<()> {
        let (ffs, _d) = init("cursor_into_works")?;
        let tree = ffs.get_dir_tree_at("")?;
        //println!("tree is {:?}", tree);
        let cursor_result_1 = tree.get_at_path("beta/gamma/abook1.m4b").unwrap();
        if let CursorIntoItem::File(a) = cursor_result_1 {
            assert!(a.file_name == "abook1.m4b");
        } else {
            panic!("The thing returned was not a File");
        };
        let cursor_result_2 = tree.get_at_path("pico8/celeste").unwrap();
        if let CursorIntoItem::Dir(a) = cursor_result_2 {
            assert!(a.dirpath == "pico8/celeste/");
        } else {
            panic!("The thing returned was not a Dir");
        };
        Ok(())
    }

    #[test]
    fn tests_recursive_search_works() -> Result<()> {
        let (ffs, _d) = init("tests_recursive_search")?;
        let tree = ffs.get_dir_tree_at("")?;
        let res1 = tree.search_files_with_names_matching_pattern_recursive(r"*.p8.png")?;
        println!("res is {:?}", res1);
        assert_eq!(res1.len(), 6);
        Ok(())
    }

    #[test]
    fn tests_single_dir_search_works() -> Result<()> {
        let (ffs, _d) = init("tests_single_dir_search")?;
        let tree = ffs.get_dir_tree_at("pico8/")?;
        let res1 = tree.search_files_with_names_matching_pattern(r"*.p8.png")?;
        println!("res is {:?}", res1);
        assert_eq!(res1.len(), 4);
        Ok(())
    }

    #[test]
    fn tests_flattening_works() -> Result<()> {
        let (ffs, _d) = init("tests_flattening")?;
        let tree = ffs.get_dir_tree_at("beta/")?;
        let res1 = tree.flatten()?;
        println!("res is {:?}", res1);
        assert_eq!(res1.len(), 5);
        Ok(())
    }

    #[test]
    fn tests_filtered_flattening_works() -> Result<()> {
        let (ffs, _d) = init("tests_flattening_filtered")?;
        let tree = ffs.get_dir_tree_at("beta/")?;
        let res1 = tree.glob(r"**/*.md")?;
        println!("res is {:?}", res1);
        assert_eq!(res1.len(), 1);
        Ok(())
    }
}
