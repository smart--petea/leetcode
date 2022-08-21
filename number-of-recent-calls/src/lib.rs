/*Variant 1
#[derive(Debug, Default)]
struct RecentCounter {
    times: Vec<i32>
}

impl RecentCounter {

    fn new() -> Self {
        RecentCounter::default()    
    }
    
    fn ping(&mut self, t: i32) -> i32 {
       self.times.push(t); 

       let min = t - 3000;
       let max = t;

       let mut counter = 0i32;
       for &time in &self.times {
           if time > max {
               return counter;
           }

           if time < min {
               continue;
           }

           counter += 1;
       }

       return counter;
    }
}
*/

use std::collections::VecDeque;

#[derive(Debug, Default)]
struct RecentCounter {
    times: VecDeque<i32>
}

impl RecentCounter {

    fn new() -> Self {
        RecentCounter::default()    
    }
    
    fn ping(&mut self, t: i32) -> i32 {
       self.times.push_back(t); 

       let min = t - 3000;
       loop {
           match self.times.pop_front() {
               Some(v) if v < min => continue,
               Some(v) => {
                   self.times.push_front(v);
                   return self.times.len() as i32;
               }
               None => panic!("the times vector can't be empty")
           }
       }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = RecentCounter::new();
        assert_eq!(obj.ping(1), 1);
        assert_eq!(obj.ping(100), 2);
        assert_eq!(obj.ping(3001), 3);
        assert_eq!(obj.ping(3002), 3);
    }
}
