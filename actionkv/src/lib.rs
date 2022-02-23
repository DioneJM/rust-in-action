use std::collections::HashMap;
use std::fmt::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use serde_derive::{Deserialize, Serialize};

// ByteStr is to &str what ByteString is to Vec<u8>
type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    file: File,
    pub index: HashMap<ByteString, u64>
}

impl ActionKV {
    pub fn open(file_path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)?;
        let index = HashMap::new();
        Ok(ActionKV { file, index })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.file);
        loop {
             let position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        }
                        _ => return Err(err)
                    }
                }
            };
            self.index.insert(kv.key, position);
        }

        Ok(())
    }

    fn process_record<R: Read>(record: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = record.read_u32::<LittleEndian>()?;
        let key_len = record.read_u32::<LittleEndian>()?;
        let value_len = record.read_u32::<LittleEndian>()?;
        let data_len = key_len + value_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        {
            record.by_ref()
                .take(data_len as u64)
                .read_to_end(&mut data)?;
        }

        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = crc::crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!(
                "Data corruption encountered ({:08x} != {:08x})",
                checksum,
                saved_checksum
            );
        }

        let value = data.split_off(key_len as usize);
        let key = data;
        Ok( KeyValuePair { key, value })
    }
    pub fn get(&self, _key: &String) -> Option<String> {
        todo!()
    }

    pub fn delete(&self, _key: &String) -> Result<(), Error> {
        todo!()
    }
    pub fn insert(&self, _key: &String, _value: &String) -> Result<String, Error> {
        todo!()
    }

    pub fn update(&self, _key: &String, _value: &String) -> Result<String, Error> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
