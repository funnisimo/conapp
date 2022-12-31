use crate::{app::File, console};
use std::collections::HashMap;

struct AsyncFile(String, File, Option<Vec<u8>>);

#[derive(Debug)]
pub enum LoadError {
    OpenError(std::io::Error),
    ReadError(std::io::Error),
}

#[derive(Default)]
/// This provides a common way to load files for both native and web targets
pub struct FileLoader {
    files_to_load: HashMap<usize, AsyncFile>,
    seq: usize,
}

impl FileLoader {
    pub fn new() -> Self {
        Default::default()
    }
    /// request to load a file. returns an id you can use with other methods
    pub fn load_file(&mut self, path: &str) -> Result<usize, LoadError> {
        crate::console(format!("loading file - {}", path));
        match open_file(path) {
            Ok(mut f) => {
                console(format!("file open - {}", path));
                if f.is_ready() {
                    match f.read_binary() {
                        Ok(buf) => {
                            self.files_to_load
                                .insert(self.seq, AsyncFile(path.to_owned(), f, Some(buf)));
                            self.seq += 1;
                            Ok(self.seq - 1)
                        }
                        Err(e) => Err(LoadError::ReadError(e)),
                    }
                } else {
                    crate::console(format!("loading async file {}", path));
                    self.files_to_load
                        .insert(self.seq, AsyncFile(path.to_owned(), f, None));
                    self.seq += 1;
                    Ok(self.seq - 1)
                }
            }
            Err(e) => Err(LoadError::OpenError(e)),
        }
    }

    fn load_file_async(&mut self) -> bool {
        for (_, f) in self.files_to_load.iter_mut() {
            if f.1.is_ready() && f.2.is_none() {
                match f.1.read_binary() {
                    Ok(buf) => {
                        f.2 = Some(buf);
                    }
                    Err(e) => panic!("could not load async file {} : {}", f.0, e),
                }
            }
        }
        true
    }

    /// return true if the file is ready in memory
    pub fn check_file_ready(&mut self, id: usize) -> bool {
        self.load_file_async();
        if let Some(f) = self.files_to_load.get(&id) {
            return f.2.is_some();
        }
        false
    }

    /// retrieve the file content
    pub fn get_file_content(&mut self, id: usize) -> Vec<u8> {
        let mut f = self.files_to_load.remove(&id).unwrap();
        f.2.take().unwrap()
    }
}

fn open_file(filename: &str) -> Result<crate::app::File, std::io::Error> {
    let ffilename = filename.to_owned();
    crate::app::FileSystem::open(&ffilename)
}
