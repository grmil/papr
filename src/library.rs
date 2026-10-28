use anyhow::Result;
use biblatex::{Bibliography, ChunksExt, Entry};
use std::collections::HashMap;
use std::{fs, path::PathBuf};

pub struct Paper {
    citation: Entry,

    pub key: String,
    pub title: String,
    pub authors: Vec<String>,
    pub year: Option<i32>,
    pub doi: Option<String>,
    pub tags: Vec<String>,
}

impl Paper {
    // Unwrap a bitex entry and save it to the paper struct
    pub fn from_entry(entry: Entry) -> Self {
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
        let text = fs::read_to_string(&path)?;

        let papers: HashMap<String, Paper> = Bibliography::parse(&text)?
            .into_vec()
            .into_iter()
            .map(Paper::from_entry)
            .map(|p| (p.key.clone(), p))
            .collect();

        Ok(Self { path, papers })
    }

    // Save the loaded bibtex entries to a file
    pub fn save(&self) -> Result<()> {
        let mut bib = Bibliography::new();
        for p in self.papers.values() {
            bib.insert(p.citation.clone());
        }

        let text = bib.to_biblatex_string();

        let tmp = self.path.with_extension("bib.tmp");
        fs::write(&tmp, text)?;
        fs::rename(&tmp, &self.path)?;

        Ok(())
    }

    // Combine two bibliographies
    fn merge(&mut self, bib: Bibliography) -> Result<()> {
        let entries = bib.into_vec();
        for entry in entries {
            let paper = Paper::from_entry(entry);
            self.papers.insert(paper.key.clone(), paper);
        }

        Ok(())
    }

    pub fn add_from_bibtex(&mut self, src: &str) -> Result<()> {
        let bib = Bibliography::parse(src)?;
        self.merge(bib)?;
        Ok(())
    }

    pub fn add_from_path(&mut self, path: PathBuf) -> Result<()> {
        let src = fs::read_to_string(&path)?;
        let bib = Bibliography::parse(&src)?;
        self.merge(bib)?;
        Ok(())
    }

    // Only takes doi, manaully appends the stuff for address lookup
    pub fn add_from_doi(&mut self, doi: &str) -> Result<()> {
        let url = format!("https://doi.org/{doi}");   

        let src = reqwest::blocking::Client::new()
            .get(url)
            .header("Accept", "application/x-bibtex")
            .send()?
            .text()?;

        let bib = Bibliography::parse(&src)?;
        self.merge(bib)?;
        Ok(())
    }
}
