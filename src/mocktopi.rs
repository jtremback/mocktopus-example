#![cfg_attr(test, feature(proc_macro))]

#[cfg(test)]
use mocktopus::macros::*;

#[cfg_attr(test, mockable)]
mod storage_mod {
    pub fn new_fish(fishname: &str) -> Result<(), String>{
        println!("making new fish in real db with name {}", fishname);
        Ok(())
    }
}

use self::storage_mod::new_fish;

struct Logic {}

impl Logic {
    pub fn populate_aquarium (&mut self, fishnames: Vec<&str>) -> Result<(), String> {
        for name in fishnames {
            new_fish(name)?;
        }
        Ok(())
    }
}

mod logic_mod {
    use super::storage_mod::new_fish;
    pub fn populate_aquarium (fishnames: Vec<&str>) -> Result<(), String> {
        for name in fishnames {
            new_fish(name)?;
        }
        Ok(())
    }
}

struct StorageStruct {}

#[cfg_attr(test, mockable)]
impl StorageStruct {
    pub fn new_fish(&mut self, fishname: &str) -> Result<(), String>{
        println!("making new fish in real db with name {}", fishname);
        Ok(())
    }
}

struct LogicWithStorage {
    pub storage: StorageStruct
}

impl LogicWithStorage {
    pub fn populate_aquarium (&mut self, fishnames: Vec<&str>) -> Result<(), String> {
        for name in fishnames {
            self.storage.new_fish(name)?;
        }
        Ok(())
    }
}

// This tests a struct with methods using a mocked module
#[cfg(test)]
mod test_logic_struct_storage_mod {
    use super::*;
    use mocktopus::mocking::*;

    #[test]
    fn no_mock() {
        let mut logic = Logic {};

        logic.populate_aquarium(vec!["huey", "bingo"]).unwrap();
    }

    #[test]
    fn happy_mock() {
        storage_mod::new_fish.mock_safe(|fishname| {
            println!("making new fish in fake db with name {}", fishname);
            MockResult::Return(Ok(()))
        });

        let mut logic = Logic {};

        logic.populate_aquarium(vec!["huey", "bingo"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn sad_mock() {
        let mut counter = 0;

        storage_mod::new_fish.mock_safe(move |fishname| {
            counter = counter + 1;
            if counter < 3 {
                println!("making new fish #{} in fake db with name {}", counter, fishname);
                MockResult::Return(Ok(()))
            } else {
                MockResult::Return(Err("too many fish".into()))
            }
        });

        let mut logic = Logic {};

        logic.populate_aquarium(vec!["huey", "bingo", "rog"]).unwrap();
    }
}



// This tests a module using a mocked module
#[cfg(test)]
mod test_logic_mod_storage_mod {
    use super::*;
    use mocktopus::mocking::*;

    #[test]
    fn no_mock() {
        logic_mod::populate_aquarium(vec!["huey", "bingo"]).unwrap();
    }

    #[test]
    fn happy_mock() {
        storage_mod::new_fish.mock_safe(|fishname| {
            println!("making new fish in fake db with name {}", fishname);
            MockResult::Return(Ok(()))
        });

        logic_mod::populate_aquarium(vec!["huey", "bingo"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn sad_mock() {
        let mut counter = 0;

        storage_mod::new_fish.mock_safe(move |fishname| {
            counter = counter + 1;
            if counter < 3 {
                println!("making new fish #{} in fake db with name {}", counter, fishname);
                MockResult::Return(Ok(()))
            } else {
                MockResult::Return(Err("too many fish".into()))
            }
        });

        logic_mod::populate_aquarium(vec!["huey", "bingo", "rog"]).unwrap();
    }
}



// This tests a struct with methods using a mocked struct with methods
mod test_logic_struct_storage_struct {
    use super::*;
    use mocktopus::mocking::*;

    #[test]
    fn no_mock() {
        let mut logic = LogicWithStorage {
            storage: StorageStruct {}
        };

        logic.populate_aquarium(vec!["huey", "bingo"]).unwrap();
    }

    #[test]
    fn happy_mock() {
        StorageStruct::new_fish.mock_safe(|_, fishname| {
            println!("making new fish in fake db with name {}", fishname);
            MockResult::Return(Ok(()))
        });

        let mut logic = LogicWithStorage {
            storage: StorageStruct {}
        };

        logic.populate_aquarium(vec!["huey", "bingo"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn sad_mock() {
        let mut counter = 0;

        StorageStruct::new_fish.mock_safe(move |_, fishname| {
            counter = counter + 1;
            if counter < 3 {
                println!("making new fish #{} in fake db with name {}", counter, fishname);
                MockResult::Return(Ok(()))
            } else {
                MockResult::Return(Err("too many fish".into()))
            }
        });

        let mut logic = LogicWithStorage {
            storage: StorageStruct {}
        };

        logic.populate_aquarium(vec!["huey", "bingo", "rog"]).unwrap();
    }
}
