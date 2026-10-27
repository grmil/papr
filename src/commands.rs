use crate::LIBRARY_PATH;
use crate::library::{Library, Paper};
use anyhow::Result;
use std::{fs, path::Path};

pub fn add(doi: &str) -> Result<()> {
    let mut library = Library::load(LIBRARY_PATH.into())?;

    // Scan the web and scrape a citation
    // TODO: implement pull_from_doi()
    //let src: String = pull_from_doi(&doi)?;

    // Add entry to the main bibliography
    //library.add_entry(&src)?;

    Ok(())
}

pub fn import(entry: &Path) -> Result<()> {
    let mut library = Library::load(LIBRARY_PATH.into())?;

    // Load the bibliography file into a bibliography object
    let src = fs::read_to_string(&entry)?;

    // Add entry to the main bibliography
    library.add_entry(&src)?;

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
