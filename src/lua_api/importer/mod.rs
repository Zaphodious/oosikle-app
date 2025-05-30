use anyhow::Result;
use relative_path::RelativePathBuf;
use std::path::{Path, PathBuf};

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

    pub fn add_file(mut self, file: RelativePathBuf) -> Self {
        self.files.push(file);
        self
    }

    pub fn add_files(mut self, files: &mut Vec<RelativePathBuf>) -> Self {
        self.files.append(files);
        self
    }

    pub fn adjust_root_considering_files(mut self) -> Result<Self> {
        Ok(self)
    }
}

#[cfg(test)]
mod file_import_tests {
    use super::*;

    #[test]
    fn tests_that_root_adjustment_works() -> Result<()> {
        let manifest = ShippingManifest::new(PathBuf::from("")).add_files(&mut vec![
            RelativePathBuf::from("/media/videogame/pico8/thing1.png"),
            RelativePathBuf::from("/media/videogame/pico8/thing2.png"),
            RelativePathBuf::from("/media/videogame/pico8/thing3.png"),
            RelativePathBuf::from("/media/videogame/mastersystem/thing1.png"),
            RelativePathBuf::from("/media/audio/music/blind_and_frozen.mp3"),
        ]).adjust_root_considering_files()?;
        assert!(manifest.root_dir == PathBuf::from("/media"));
        Ok(())
    }
}
