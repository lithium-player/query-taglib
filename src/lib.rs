extern crate liquery;
extern crate taglib;

use std::path::Path;

use liquery::Queryable;
use taglib::{File, Tag};

pub struct AudioFile {
    file: File,
}

impl AudioFile {
    // FIXME: proper error management
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        match File::new(path.as_ref().to_str().unwrap()) {
            Ok(file) => {
                if !file.is_valid() {
                    println!("not valid");
                }
                Ok(AudioFile { file: file })
            }
            Err(e) => {
                println!("{:?}", e);
                Err(())
            } // FIXME
        }
    }
}

impl Queryable for AudioFile {
    fn query(&self, key: &str) -> Option<String> {
        match self.file.tag() {
            Ok(tag) => {
                match key {
                    "title" => Some(tag.title()),
                    "artist" => Some(tag.artist()),
                    "album" => Some(tag.album()),
                    "comment" => Some(tag.comment()),
                    "genre" => Some(tag.genre()),
                    "year" => Some(format!("{}", tag.year())),
                    "track" => Some(format!("{}", tag.track())),
                    _ => None,
                }
            }
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    }
}
