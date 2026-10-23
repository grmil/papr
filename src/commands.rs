use anyhow::Result;
use std::path::Path;
use src::Library;

pub fn add(_doi: &str) -> Result<()> {
    todo!()
}

pub fn import(_entry: &Path) -> Result<()> {
    todo!()
}

pub fn list() -> Result<()> {
    let library = Library.load(path);

}

pub fn search(_query: &[String]) -> Result<()> {
    todo!()
}

pub fn remove(_key: &str) -> Result<()> {
    todo!()
}

pub fn open(_key: &str) -> Result<()> {
    todo!()
}

pub fn note(_key: &str) -> Result<()> {
    todo!()
}

pub fn export(_key: &str) -> Result<()> {
    todo!()
}
