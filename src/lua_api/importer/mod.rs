use anyhow::Result;
use base64::alphabet::URL_SAFE;
use hypertext::html_elements::div;
use mlua::serde::de;
use rayon::prelude::*;
use relative_path::{Component as rComponent, PathExt, RelativePath, RelativePathBuf};
use rust_search::{FilterExt, SearchBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs::{self, File};
use std::io::BufReader;
use std::{
    collections::HashSet,
    path::{Component as sComponent, Path, PathBuf},
};
use zip::{self, read::ZipFile};
use std::time::SystemTime;
use base64::prelude::*;

use crate::db::FileRecord;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirImportManifest {
    pub root_dir: PathBuf,
    pub items: Vec<RelativePathBuf>,
}

impl DirImportManifest {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir: root_dir,
            items: vec![],
        }
    }

    pub fn add_relative_file(mut self, file: RelativePathBuf) -> Self {
        self.items.push(file);
        self
    }

    pub fn add_relative_files(mut self, files: Vec<RelativePathBuf>) -> Self {
        for f in files {
            self.items.push(f);
        }
        self
    }

    pub fn add_file(mut self, file: PathBuf) -> Result<Self> {
        self.items.push(file.relative_to(&self.root_dir)?);
        Ok(self)
    }

    pub fn create_manifest_from_path_vec(pathvec: Vec<PathBuf>) -> Result<Self> {
        let denatured_paths: Vec<Vec<sComponent>> =
            pathvec.iter().map(|p| p.components().collect()).collect();
        println!("denatured paths are {:?}", denatured_paths);
        let mut root_accumulator = PathBuf::new();
        let mut depth: usize = 0;
        let mut has_found = false;
        loop {
            let mut last_comp: Option<&sComponent> = None;
            for thepath in &denatured_paths {
                let current_comp = thepath.get(depth);
                if current_comp == None {
                    has_found = true;
                    break;
                } else if last_comp == None {
                    last_comp = current_comp;
                } else if last_comp != current_comp {
                    println!(
                        "components are last_comp:{:?} and current_comp:{:?}",
                        last_comp, current_comp
                    );
                    has_found = true;
                    break;
                }
            }
            if has_found {
                break;
            } else {
                depth += 1;
                println!("Pushing {:?} to the root accumulator", last_comp);
                root_accumulator.push(last_comp.unwrap());
            }
        }
        println!("Root accumulator is {:?}", root_accumulator);
        Ok(Self {
            items: pathvec
                .iter()
                .map(|p| p.relative_to(&root_accumulator).unwrap())
                .filter(|p| !p.to_string().is_empty())
                .collect(),
            root_dir: root_accumulator,
        })
    }

    pub fn create_from_dir_on_disk(location: PathBuf) -> Result<Self> {
        let s: Vec<_> = SearchBuilder::default()
            .location(&location)
            .search_input(r#".*"#)
            .ext("*")
            .ignore_case()
            .build()
            .map(|e| PathBuf::from(e))
            .filter(|e| e.is_file())
            .collect();
        //let _rootdir = s.pop(); // first is gonna be the qualified root dir itself
        Ok(Self::create_manifest_from_path_vec(s)?)
    }

    pub fn construct_container(mut self, import_session_id: &str) -> Result<InboundFileRecordContainer> {
        let root_dir = self.root_dir.clone();
        let records = self
            .items
            .into_iter()
            .map(|p| (p.clone(), p.to_path(&root_dir)))
            .map(|(rp, pb)| (rp, pb.canonicalize()))
            .filter(|(rp, pr)| (&pr).is_ok() && (&pr).as_ref().unwrap().is_file())
            .map(|(r, p)| (r, p.unwrap()))
            .map(|(r, p)| {
                let mut hasher = blake3::Hasher::new();
                match hasher.update_mmap_rayon(&p) {
                    Ok(h) => (r, p, Some(hasher.finalize())),
                    Err(e) => (r, p, None),
                }
            })
            .map(|(r, p, h)| {
                let hashstr = match h {
                    Some(hash) => hash.to_string(),
                    None => "".to_string(),
                };
                let full_filename = p.file_name().unwrap().to_str().unwrap();
                let (_, fileext) = match full_filename.split_once(".") {
                    Some(t) => t,
                    None => (full_filename, ""),
                };
                let filesize = if let Ok(openfile) = File::open(&p) {
                    if let Ok(md) = openfile.metadata() {
                        md.len()
                    } else {
                        0
                    }
                } else {
                    0
                };
                let vfspathroot = RelativePath::new(import_session_id).join(r);
                FileRecord {
                    file_uuid: "".into(),
                    file_vfs_path: vfspathroot.parent().unwrap().to_string(),
                    file_size_bytes: filesize,
                    file_read_only: false,
                    file_name: full_filename.into(),
                    file_extension_tag: fileext.into(),
                    file_encoding: "".into(),
                    file_dir_path: p.parent().unwrap().to_str().unwrap().to_string(),
                    file_hash: hashstr,
                    file_deleted: false,
                    media_type_override_id: None,
                }
            }).collect();
            Ok(InboundFileRecordContainer {
                root_dir,
                import_session_id: import_session_id.to_string(),
                records
            })
    }
}

pub fn make_import_id_with_time() -> Result<String>{
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;
    let ms = since_epoch.as_secs_f64();
    let sixtyfour = BASE64_URL_SAFE_NO_PAD.encode(ms.to_be_bytes());
    Ok(sixtyfour)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InboundFileRecordContainer {
    root_dir: PathBuf,
    import_session_id: String,
    records: Vec<FileRecord>,
}

impl InboundFileRecordContainer {
    pub fn give_ids_to_records(&mut self) -> &mut Self {
        (&mut self.records).into_iter().for_each(|r| {
            let id = Uuid::now_v7();
            r.file_uuid = id.simple().to_string();
        }); 
        return self;
    }
}

#[cfg(test)]
mod file_import_tests {
    use super::*;

    #[test]
    fn tests_making_manifest_from_path_vec() -> Result<()> {
        let pathvec = vec![
            PathBuf::from("c:/media/videogame/pico8/thing1.png"),
            PathBuf::from("c:/media/videogame/pico8/thing1.png"),
            PathBuf::from("c:/media/videogame/pico8/thing2.png"),
            PathBuf::from("c:/media/videogame/pico8/thing3.png"),
            PathBuf::from("c:/media/videogame/snes/echo.sns"),
            PathBuf::from("c:/media/videogame/mastersystem/thing1.png"),
        ];
        let manifest = DirImportManifest::create_manifest_from_path_vec(pathvec)?;
        println!("manifest is {:?}", manifest);
        println!(
            "root dir is {:?} while the thing from is {:?}",
            manifest.root_dir,
            PathBuf::from("c:/media/videogame")
        );
        assert!(manifest.root_dir == PathBuf::from("c:/media/videogame"));
        Ok(())
    }

    const IMPORT_PATH_STR: &str = "./src/testing_data/import_test";

    //#[cfg(target_os = "unix")]
    #[test]
    fn tests_creates_from_dir_on_disk() -> Result<()> {
        let manifest =
            DirImportManifest::create_from_dir_on_disk(IMPORT_PATH_STR.into())?;
        println!("manifest is {:?}", manifest);
        assert!(manifest.items.len() == 6);
        Ok(())
    }

    #[test]
    fn tests_makes_file_records_for_things() -> Result<()> {
        let manifest =
            DirImportManifest::create_from_dir_on_disk(IMPORT_PATH_STR.into())?;
        let old_root = manifest.root_dir.clone();
        let old_len = manifest.items.len();
        let inbound_container = manifest.construct_container(make_import_id_with_time()?.as_str())?;
        println!("the container is {:?}", inbound_container);
        assert!(inbound_container.records.len() == old_len);
        assert!(inbound_container.root_dir == old_root);
        Ok(())
    }
    #[test]
    fn tests_uuids_are_created_correctly() -> Result<()> {
        let manifest =
            DirImportManifest::create_from_dir_on_disk(IMPORT_PATH_STR.into())?;
        let old_len = manifest.items.len();
        let mut inbound_container = manifest.construct_container(make_import_id_with_time()?.as_str())?;
        inbound_container.give_ids_to_records();
        let uuids: HashSet<String> = inbound_container.records.into_iter().map(|r| r.file_uuid).collect();
        println!("uuids are {:?}", uuids);
        assert!(uuids.len() == old_len);
        Ok(())
    }
}
