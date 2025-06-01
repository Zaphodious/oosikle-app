use anyhow::Result;
use hypertext::html_elements::div;
use mlua::serde::de;
use relative_path::{Component as rComponent, PathExt, RelativePathBuf};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    path::{Component as sComponent, Path, PathBuf},
};
use rust_search::{FilterExt, SearchBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingManifest {
    pub root_dir: PathBuf,
    pub files: Vec<RelativePathBuf>,
}

impl ShippingManifest {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,
            files: vec![],
        }
    }

    pub fn add_relative_file(mut self, file: RelativePathBuf) -> Self {
        self.files.push(file);
        self
    }

    pub fn add_relative_files(mut self, files: &mut Vec<RelativePathBuf>) -> Self {
        self.files.append(files);
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
                    println!("components are last_comp:{:?} and current_comp:{:?}", last_comp, current_comp);
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
            files: pathvec
                .iter()
                .map(|p| p.relative_to(&root_accumulator).unwrap())
                .filter(|p| !p.to_string().is_empty())
                .collect(),
            root_dir: root_accumulator,
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
            .collect();
        //let _rootdir = s.pop(); // first is gonna be the qualified root dir itself
        Ok(Self::create_manifest_from_path_vec(s)?)
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
        let manifest = ShippingManifest::create_manifest_from_path_vec(pathvec)?;
        println!("manifest is {:?}", manifest);
        println!("root dir is {:?} while the thing from is {:?}", manifest.root_dir, PathBuf::from("c:/media/videogame"));
        assert!(manifest.root_dir == PathBuf::from("c:/media/videogame"));
        Ok(())
    }

    #[test]
    fn tests_creates_from_dir_on_disk() -> Result<()> {
        let manifest = ShippingManifest::create_from_dir_on_disk("~/dev/oosikle-app/src/testing_data/import_test".into())?;
        println!("manifest is {:?}", manifest);
        assert!(manifest.files.len() == 5);
        Ok(())
    }
}
