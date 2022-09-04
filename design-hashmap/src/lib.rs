#[derive(Debug)]
struct MyHashMap {
    root: Option<Box<MyHashCell>>,
}

#[derive(Debug)]
struct MyHashCell {
    key: i32,
    val: i32,
    left: Option<Box<MyHashCell>>,
    right: Option<Box<MyHashCell>>,
}

impl MyHashCell {
    fn new(key: i32, val: i32) -> Self {
        MyHashCell {
            key: key,
            val: val,
            left: None,
            right: None,
        }
    }

    fn new_some_box(key: i32, val: i32) -> Option<Box<MyHashCell>>{
        Some(Box::new(MyHashCell::new(key, val)))
    }
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            root: None,
        }
    }


    fn put(&mut self, key: i32, value: i32) {
        if self.root.is_none() {
            self.root = MyHashCell::new_some_box(key, value);
            return;
        }

        if MyHashMap::inner_put(self.root.as_mut(), key, value) {
            return;
        }
    }

    fn inner_put(mut root: Option<&mut Box<MyHashCell>>, key: i32, value: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let root_box = root.unwrap();

        if root_box.key == key {
            root_box.val = value;
            return true;
        }


        if root_box.key > key {
            if root_box.left.is_some() {
                return MyHashMap::inner_put(root_box.left.as_mut(), key, value);
            } else {
                root_box.left = MyHashCell::new_some_box(key, value);
                return true;
            }
        }

        if root_box.right.is_some() {
            return MyHashMap::inner_put(root_box.right.as_mut(), key, value);
        }

        root_box.right = MyHashCell::new_some_box(key, value);
        return true;
    }
    
    fn get(&self, key: i32) -> i32 {
        match MyHashMap::inner_get(&self.root, key) {
            Some(val) => val,
            None => -1,
        }
    }

    fn inner_get(root: &Option<Box<MyHashCell>>, key: i32) -> Option<i32> {
        if let Some(root_box) = root {
            if root_box.key == key {
                return Some(root_box.val);
            }

            if root_box.key < key {
                return MyHashMap::inner_get(&root_box.right, key);
            }

            return MyHashMap::inner_get(&root_box.left, key);
        }

        return None;
    }
    
    fn remove(&mut self, key: i32) {
        self.put(key, -1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut my_hash_map = MyHashMap::new();
        //assert_eq!(myHashMap.get(1), -1);
        //assert_eq!(myHashMap.get(2), -1);

        my_hash_map.put(1, 1);
        //assert_eq!(myHashMap.get(1), 1);
        //assert_eq!(myHashMap.get(2), -1);

        my_hash_map.put(2, 2);
        //assert_eq!(myHashMap.get(1), 1);
        //assert_eq!(myHashMap.get(2), 2);

        my_hash_map.put(4, 4);
        my_hash_map.put(3, 3);
        assert_eq!(my_hash_map.get(1), 1);
        assert_eq!(my_hash_map.get(2), 2);
        assert_eq!(my_hash_map.get(3), 3);
        assert_eq!(my_hash_map.get(4), 4);
        assert_eq!(my_hash_map.get(5), -1);

        my_hash_map.remove(2);
        assert_eq!(my_hash_map.get(2), -1);
    }
}
