use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, Write};

use crate::row;
use crate::table;

pub struct Pager {
    file_descriptor: File,
    file_length: u64,
    pages_cached: usize,
    pages: Vec<table::Page>,
}

impl Pager {
    pub fn new(filename: String) -> Pager {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .expect("DB file did not exist");

        let file_length = file.metadata()
            .expect("could not read db file metadata")
            .len();

        Pager {
            file_descriptor: file,
            file_length: file_length,
            pages: vec![],
            pages_cached: 0,
        }
    }

    pub fn close(&mut self) {
        let mut i = 0;
        for page in self.pages.iter() {
            self.file_descriptor
                .seek(io::SeekFrom::Start(i))
                .expect("Unable to seek to position");

            self.file_descriptor.write(&page.data);

            i += table::PAGE_SIZE as u64;
        }

        self.file_descriptor.sync_data()
            .expect("unable to write file");
    }

    pub fn num_rows(&self) -> usize {
        (self.file_length / row::ROW_SIZE as u64) as usize
    }

    pub fn get_page(&mut self, page_idx: usize) -> &mut table::Page {
        while page_idx >= self.pages.len() {
            self.load_page(self.pages.len())
        }

        &mut self.pages[page_idx]
    }

    pub fn load_page(&mut self, page_num: usize) {
        // TODO: check for cached pages being null initially?
        // i.e. make page an option with pointer to row
        let mut page_buffer = vec![0; table::PAGE_SIZE];
        let page_offset = (page_num * table::PAGE_SIZE) as u64;

        if page_offset >= self.file_length {
            // make a empty page
            self.pages.push(table::Page {
                data: [0; table::PAGE_SIZE],
            });

            return
        }

        let offset = self.file_descriptor
            .seek(io::SeekFrom::Start(page_offset))
            .expect("Unable to seek to position");

        if offset != page_offset {
            panic!("seek position did not match page offset");
        }

        let bytes_read = self.file_descriptor
            .read(page_buffer.as_mut())
            .expect("Unable to read bytes from page");

        if bytes_read != page_buffer.len() {
            panic!("Unable to read a full page from file");
        }

        self.pages[page_num] = table::Page {
            data: page_vec_to_array(page_buffer),
        };

        self.pages_cached += 1;
    }
}

fn page_vec_to_array(v: Vec<u8>) -> [u8; table::PAGE_SIZE] {
    let mut ret = [0; table::PAGE_SIZE];

    let mut i = 0;
    for b in v.iter() {
        ret[i] = *b;
        i += 1;
    }

    ret

}
