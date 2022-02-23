use std::fmt::Error;
use std::path::Path;

pub struct ActionKV {

}

impl ActionKV {
    pub fn open(_file_path: &Path) -> Result<Self, Error> {
        todo!()
    }

    pub fn load(&mut self) -> Result<(), Error> {
        todo!()
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
