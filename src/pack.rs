use binread::BinRead;
use binwrite::BinWrite;
use std::io::Cursor;

#[derive(Clone)]
pub struct Packed {
    pub files: Vec<Vec<u8>>,
}

impl Packed {
    fn file_size(&self) -> u32 {
        let header_length = (self.files.len() * 4) as u32;
        let files_length = self.files.iter().fold(0, |pv, cv| pv + cv.len()) as u32;

        header_length + files_length
    }
}

impl From<Vec<u8>> for Packed {
    fn from(file: Vec<u8>) -> Self {
        let mut reader = Cursor::new(&file);
        let mut files: Vec<Vec<u8>> = Vec::new();

        let f0 = u32::read(&mut reader).unwrap();

        if f0 == 0 {
            return Packed { files };
        }

        let mut offsets: Vec<u32> = Vec::new();

        offsets.push(f0);

        for _ in 0..(f0 / 4) - 1 {
            offsets.push(u32::read(&mut reader).unwrap());
        }

        for i in 0..offsets.len() - 1 {
            files.push(file[offsets[i] as usize - 1..offsets[i + 1] as usize - 1].into());
        }

        files.push(file[*offsets.last().unwrap() as usize - 1..].into());

        Packed { files }
    }
}

impl Into<Vec<u8>> for Packed {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut i: u32 = (self.files.len() * 4) as u32 + 1;

        for file in &self.files {
            i.write(&mut result).unwrap();
            i += file.len() as u32;
        }

        for file in self.files {
            result.extend(file);
        }

        result
    }
}
