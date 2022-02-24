use std::collections::HashMap;
use std::fmt::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;
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
    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
       let position = match self.index.get(key) {
           None => return Ok(None),
           Some(position) => *position
       };

        let kv = self.get_at(position)?;
        Ok(Some(kv.value))
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut file = BufReader::new(&mut self.file);
        file.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut file)?;

        Ok(kv)
    }

    pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut file = BufReader::new(&mut self.file);
        let mut found: Option<(u64, ByteString)> = None;

        loop {
            let position = file.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut file);
            let kv =  match maybe_kv {
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
            if kv.key == target {
                found = Some((position, kv.value));
            }

            // Need to keep looping until EOF incase the key has been overwritten
        };

        Ok(found)
    }

    pub fn delete(&self, _key: &String) -> Result<(), Error> {
        todo!()
    }
    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);

        Ok(())
    }

    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut file = BufWriter::new(&mut self.file);

        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        for byte in key {
            tmp.push(*byte);
        }

        for byte in value {
            tmp.push(*byte);
        }

        let checksum = crc32::checksum_ieee(&tmp);

        let next_byte = SeekFrom::End(0);
        let current_position = file.seek(SeekFrom::Current(0))?;
        file.seek(next_byte)?;
        file.write_u32::<LittleEndian>(checksum)?;
        file.write_u32::<LittleEndian>(key_len as u32)?;
        file.write_u32::<LittleEndian>(val_len as u32)?;
        file.write_all(&tmp)?;

        Ok(current_position)
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
