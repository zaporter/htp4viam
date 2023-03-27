use std::path::PathBuf;


pub struct RcFolder {
    path: PathBuf,
}

impl RcFolder {
    pub fn new(path: PathBuf) -> Self {
        Self { path } 
    }
    pub fn get_path(&self) -> & PathBuf {
        &self.path
    }
}

