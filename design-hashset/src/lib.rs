struct MyHashSet {
    storage: Vec<i32>
}

impl MyHashSet {

    fn new() -> Self {
       MyHashSet {
           storage: Vec::new(),
       }
    }
    
    fn add(&mut self, key: i32) {
        if self.contains(key) {
            return;
        }

        self.storage.push(key);
    }
    
    fn remove(&mut self, key: i32) {
        if self.contains(key) == false {
            return;
        }

        for i in 0..self.storage.len() {
            if self.storage[i] == key {
                self.storage.remove(i);
                return;
            }
        }
    }
    
    fn contains(&self, key: i32) -> bool {
        if self.storage.len() == 0 {
            return false;
        }

        for i in 0..self.storage.len() {
            if self.storage[i] == key {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut myHashSet = MyHashSet::new();
        assert_eq!(myHashSet.storage.len(), 0);

        myHashSet.add(1);
        assert_eq!(myHashSet.storage.len(), 1);

        myHashSet.add(2);
        assert_eq!(myHashSet.storage.len(), 2);

        assert_eq!(myHashSet.contains(1), true);
        assert_eq!(myHashSet.contains(3), false);

        myHashSet.add(2);
        assert_eq!(myHashSet.storage.len(), 2);

        assert_eq!(myHashSet.contains(2), true);

        myHashSet.remove(2);
        assert_eq!(myHashSet.storage.len(), 1);

        assert_eq!(myHashSet.contains(2), false);
    }
}
