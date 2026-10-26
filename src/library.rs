use anyhow::Result;
use biblatex::{Bibliography, ChunksExt};
use std::collections::HashMap;
use std::{fs, path::PathBuf};

pub struct Paper {
    citation: biblatex::Entry,

    pub key: String,
    pub title: String,
    pub authors: Vec<String>,
    pub year: Option<i32>,
    pub doi: Option<String>,
    pub tags: Vec<String>,
}

impl Paper {
    // Unwrap a bitex entry and save it to the paper struct
    pub fn from_entry(entry: biblatex::Entry) -> Self {
        let get = |name: &str| entry.get(name).map(|c| c.format_verbatim());

        let key = entry.key.clone();
        let title = get("title").unwrap_or_default();
        let authors = get("author")
            .unwrap_or_default()
            .split(" and ")
            .map(|a| a.trim().to_string())
            .collect();
        let year = get("year").and_then(|y| y.trim().parse().ok());
        let doi = get("doi");
        let tags = get("tags")
            .unwrap_or_default()
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();

        Self {
            citation: entry,
            key,
            title,
            authors,
            year,
            doi,
            tags,
        }
    }
}

pub struct Library {
    path: PathBuf,
    pub papers: HashMap<String, Paper>,
}

impl Library {
    // Pull in an convert bibtex entries from a .bib file
    pub fn load(path: PathBuf) -> Result<Self> {
        let bib = {
            let file = fs::read_to_string(&path)?;
            Bibliography::parse(&file)
        };

        let papers = bib?.into_vec().into_iter().map(Paper::from_entry).collect();
        Ok(Self { path, papers })
    }

    // Save the loaded bibtex entries to a file
    pub fn save(&self) -> Result<()> {
        let mut bib = Bibliography::new();
        for p in &self.papers {
            bib.insert(p.citation.clone());
        }

        let text = bib.to_biblatex_string();

        let tmp = self.path.with_extension("bib.tmp");
        fs::write(&tmp, text)?;
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }
}
