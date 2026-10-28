use crate::LIBRARY_PATH;
use crate::library::Library;
use anyhow::Result;
use std::path::Path;

pub fn add(src: &str) -> Result<()> {
    let mut library = Library::load(LIBRARY_PATH.into())?;

    let s = src.trim();   

    if s.starts_with('@') {
        library.add_from_bibtex(s)?;
    } else if Path::new(s).exists() {
        library.add_from_path(s.into())?;
    } else if s.starts_with("10.") && s.contains('/') {
        library.add_from_doi(s)?;
    } else {
        anyhow::bail!("not a DOI, BibTeX string, or existing file: {s}");
    }

    library.save()?;
    Ok(())
}

pub fn list() -> Result<()> {
    let library = Library::load(LIBRARY_PATH.into())?;

    for (key, p) in &library.papers {
        println!("{} | {} | {}", key, p.title, p.authors.join("; "));
    }

    Ok(())
}

pub fn search(query: &[String]) -> Result<()> {
    let library = Library::load(LIBRARY_PATH.into())?;

    // Get results from bibliography based on search query
    // TODO: implement find_entry()
    //let results: Bibliography = library.find_entry(&query)?;

    // Output results to stdout
    // TODO: finish this
    //for p in &results {
    //    todo!() 
    //}

    Ok(())
}

pub fn remove(key: &str) -> Result<()> {
    let library = Library::load(LIBRARY_PATH.into())?;

    // Remove the key from library, it will not be saved to .bib on completion
    //ibrary.remove_entry(&key)?;
    
    Ok(())
}

pub fn open(key: &str) -> Result<()> {
    let library = Library::load(LIBRARY_PATH.into())?;

    // Open the pdf based on the citation key
    //library.open_pdf(&key)?;

    Ok(())
}

pub fn note(_key: &str) -> Result<()> {
    // Open current note if possible
    // Else open a note buffer with $EDITOR
    // On save, save to notes/[key].md
    todo!()
}

pub fn export(key: &str) -> Result<()> {
    let library = Library::load(LIBRARY_PATH.into())?;

    // Print bibliography of an entry to stdout
    //library.export_bib(&key)?; 

    Ok(())
}
