use binread::BinRead;
use binwrite::BinWrite;
use std::io::Cursor;

#[derive(Clone)]
pub struct Packed {
    pub files: Vec<Vec<u8>>,
}

impl Packed {
    pub fn file_size(&self) -> usize {
        let header_length = self.files.len() * 4 + 4;
        let files_length = self.files.iter().fold(0, |pv, cv| pv + cv.len());

        header_length + files_length
    }
}

impl Packed {
    pub fn from_text(file: Vec<u8>) -> Packed {
        let mut reader = Cursor::new(&file);
        let mut files: Vec<Vec<u8>> = Vec::new();

        let length = u32::read(&mut reader).unwrap();

        if length == 0 {
            return Packed { files };
        }

        let mut offsets: Vec<u32> = Vec::new();
        for _ in 0..length {
            offsets.push(u32::read(&mut reader).unwrap());
        }

        for i in 0..offsets.len() - 1 {
            if offsets[i] >= offsets[i + 1] {
                files.push(Vec::new());
                continue;
            }

            files.push(file[offsets[i] as usize..offsets[i + 1] as usize].into());
        }

        files.push(file[*offsets.last().unwrap() as usize..].into());
        Packed { files }
    }
}

impl From<Vec<u8>> for Packed {
    fn from(file: Vec<u8>) -> Self {
        let mut reader = Cursor::new(&file);
        let mut files: Vec<Vec<u8>> = Vec::new();

        let first_offset = u32::read(&mut reader).unwrap();

        if first_offset == 0 {
            return Packed { files };
        }

        let mut offsets: Vec<u32> = Vec::new();
        for _ in 0..first_offset / 4 {
            offsets.push(u32::read(&mut reader).unwrap());
        }

        for i in 0..offsets.len() - 1 {
            if offsets[i] >= offsets[i + 1] {
                files.push(Vec::new());
                continue;
            }

            files.push(file[offsets[i] as usize..offsets[i + 1] as usize].into());
        }

        files.push(file[*offsets.last().unwrap() as usize..].into());
        Packed { files }
    }
}

impl From<Packed> for Vec<u8> {
    fn from(val: Packed) -> Self {
        let mut result = Vec::new();
        let mut i: u32 = (val.files.len() * 4) as u32 + 4;

        (val.files.len() as u32).write(&mut result).unwrap();

        for file in &val.files {
            i.write(&mut result).unwrap();
            i += file.len() as u32;
        }

        for file in val.files {
            result.extend(file);
        }

        result
    }
}
