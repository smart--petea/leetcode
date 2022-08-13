struct OrderedStream {
    ptr: usize,
    list: Vec<Option<String>>
}

impl OrderedStream {

    fn new(n: i32) -> Self {
        OrderedStream { 
            ptr: 0,
            list: vec![None; n as usize],
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let mut result = Vec::new();
        self.list[(id_key - 1) as usize] = Some(value);

        loop {
            if self.list.len() <= self.ptr {
                break;
            }

            match self.list[self.ptr] {
                Some(ref s) =>  {
                    result.push(s.clone());
                    self.ptr = self.ptr + 1;
                } 
                None => break,
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "aaaaa".to_string();
        let b = "bbbbb".to_string();
        let c = "ccccc".to_string();
        let d = "ddddd".to_string();
        let e = "eeeee".to_string();

        let mut os = OrderedStream::new(5);

        let result = os.insert(3, c.clone());
        assert_eq!(result.len(), 0);

        let result = os.insert(1, a.clone());
        assert_eq!(result, vec![a]);

        let result = os.insert(2, b.clone());
        assert_eq!(result, vec![b, c]);

        let result = os.insert(5, e.clone());
        assert_eq!(result, Vec::<String>::new());


        let result = os.insert(4, d.clone());
        assert_eq!(result, vec![d, e]);
    }
}
