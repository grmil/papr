use anyhow::Result;
use biblatex::Bibliography;
use std::{fs, io::ErrorKind, path::PathBuf};

pub struct Paper {
    citation: biblatex::Entry,

    pub title: String,
    pub authors: Vec<Person>,
    pub year: Option<i32>,
    pub doi: Option<String>,
    pub tags: Vec<String>,
}

pub struct Library {
    path: PathBuf,
    papers: Paper,
}

impl Library {

    // Pull in an convert bibtex entries from a .bib file
    pub fn load(path: PathBuf) -> Result<Self> {
        let bib = match fs::read_to_string(&path) {
            Ok(file) => Bibliography::parse(file),
            Err(e) => return Err(e.into()),
        };
        let papers = bib.into_vec().into_iter().map(Paper::from_entry).collect();
        Ok(Self { path, papers })
    }

    // Save the loaded bibtex entries to a file
    pub fn save(&self) -> Result<()> {
        let mut bib = Bibliography::new();
        for p in &self.papers {
            bib.insert(p.entry.clone());
        }

        let text = bib.to_biblatex_string();

        let tmp = self.path.with_extension("bib.tmp");
        fs::write(&tmp, text)?;
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }
}
