//! Doc for Module.

use std::cell::RefCell;

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub phone: String
}

/// Represents an address book
pub struct PhoneCatalog {
    // using RefCell to allow interior mutability
    entries: RefCell<Vec<Entry>>
}

impl PhoneCatalog {
    pub fn new() -> PhoneCatalog {
        return PhoneCatalog {entries: RefCell::new(Vec::new())}
    }

    pub fn add(&self, entry: Entry) {
        self.entries.borrow_mut().push(entry);
    }

    pub fn print(&self) {
        for entry in self.entries.borrow().iter() {
            // using debug formatting for now - call me lazy :-)
            println!("{:?}", entry);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        
    }
}