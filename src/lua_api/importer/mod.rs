use anyhow::Result;
use hypertext::html_elements::div;
use mlua::serde::de;
use relative_path::{Component as rComponent, PathExt, RelativePathBuf};
use rust_search::{FilterExt, SearchBuilder};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::BufReader;
use std::{
    collections::HashSet,
    path::{Component as sComponent, Path, PathBuf},
};
use zip::{self, read::ZipFile};

use crate::db::FileRecord;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ManifestList {
    Dir(Vec<RelativePathBuf>),
    Zip(Vec<(usize, PathBuf)>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingManifest {
    pub root_dir: PathBuf,
    pub items: Option<ManifestList>,
    pub records: Option<Vec<FileRecord>>,
}

impl ShippingManifest {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir: root_dir,
            items: Some(ManifestList::Dir(vec![])),
            records: None,
        }
    }

    pub fn add_relative_file(mut self, file: RelativePathBuf) -> Self {
        if let Some(ManifestList::Dir(items)) = (&mut self.items) {items.push(file);}
        self
    }

    pub fn add_relative_files(mut self, files: Vec<RelativePathBuf>) -> Self {
        for f in files {
            if let Some(ManifestList::Dir(items)) = (&mut self.items) {items.push(f);}
        }
        self
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
            items: Some(ManifestList::Dir(pathvec
                .iter()
                .map(|p| p.relative_to(&root_accumulator).unwrap())
                .filter(|p| !p.to_string().is_empty())
                .collect())),
            root_dir: root_accumulator,
            records: None,
        })
    }

    fn create_from_dir_on_disk(location: PathBuf) -> Result<Self> {
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

    fn create_from_zip_file_as_dir(location: PathBuf) -> Result<Self> {
        let canon = location.canonicalize()?;
        println!("Canon path is {:?}", canon);
        let file = fs::File::open(&canon)?;
        let reader = BufReader::new(file);
        let mut archive = zip::ZipArchive::new(reader)?;

        let mut path_accum= vec![];
        for i in 0..archive.len() {
            let zipfile = archive.by_index(i)?;
            if zipfile.is_file() {
                if let Some(fp) = zipfile.enclosed_name() {
                    path_accum.push((i, fp));
                }
            }
        }

        Ok(Self {
            root_dir: canon,
            items: Some(ManifestList::Zip(path_accum)),
            records: None,
        })
    }

    fn resolve_file_records(&mut self) -> Result<()> {

        Ok(())
    }

    /*
    fn resolve_file_records(mut self) -> Result<Self> {
        let accum = vec![];
        let mut zipfile = if let RootPath::ZipPath(zp) = &self.root_dir {
            let file = fs::File::open(&zp)?;
            let reader = BufReader::new(file);
            let mut archive = zip::ZipArchive::new(reader)?;
            Some(archive)
        } else {
            None
        };
        let rootpath = match &self.root_dir {
        }
        for manifest_item in self.items {
            if let ManifestItem::ZipPath(i, p) = manifest_item {
                let zp = zipfile.expect("If we are dealing with zippaths, we should have a zipfile");
                let in_zip_file = zp.by_index(i)?;
                accum.push(FileRecord {
                    file_uuid: "".into(),
                    file_name: p.file_name().unwrap().to_str().unwrap().to_string(),
                    file_size_bytes: in_zip_file.size(),
                    file_deleted: false,
                    file_read_only: true,
                    file_dir_path: 
                })

            }

        };
        Ok(self)
    } */
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
        let manifest = ShippingManifest::create_manifest_from_path_vec(pathvec)?;
        println!("manifest is {:?}", manifest);
        println!(
            "root dir is {:?} while the thing from is {:?}",
            manifest.root_dir,
            PathBuf::from("c:/media/videogame")
        );
        assert!(manifest.root_dir == PathBuf::from("c:/media/videogame"));
        Ok(())
    }

    //#[cfg(target_os = "unix")]
    #[test]
    fn tests_creates_from_dir_on_disk() -> Result<()> {
        let manifest = ShippingManifest::create_from_dir_on_disk(
            "./src/testing_data/import_test".into(),
        )?;
        println!("manifest is {:?}", manifest);
        assert!(if let Some(ManifestList::Dir(d)) = manifest.items {d.len()} else {9999} == 6);
        Ok(())
    }

    #[test]
    fn test_creates_from_zip_file_as_dir() -> Result<()> {
        let manifest = ShippingManifest::create_from_zip_file_as_dir(
            "./src/testing_data/import_test/archive_with_files.zip".into(),
        )?;
        println!("manifest is {:?}", manifest);
        assert!(if let Some(ManifestList::Zip(d)) = manifest.items {d.len()} else {9999} == 5);
        Ok(())
    }
}
