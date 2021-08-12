use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut string_sums: HashMap<String, i32>  = HashMap::new();

        for (index1, item) in list1.iter().enumerate() {
            string_sums.insert(item.to_string(), -(index1 as i32));
        }

        let mut min: i32 = (list1.len() + list2.len()) as i32;
        let mut sum: i32 = 0;
        for (index2, item) in list2.iter().enumerate() {
            match string_sums.get(item) {
                Some(index1) => {
                    sum = -index1 + index2 as i32;
                    if min >= sum {
                        min = sum;
                    }

                    string_sums.insert(item.to_string(), sum);
                },
                _ => (),
            }
        }

        let mut result: Vec<String> = Vec::new();
        for (item, sum) in string_sums.iter() {
            println!("item={} sum={}", item, sum);
            if *sum != min {
                continue;
            }

            result.push(item.to_string());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn find_restaurant() {
        let list1: Vec<String> = vec!["Shogun".to_string(), "Tapioca Express".to_string(), "Burger King".to_string(),"KFC".to_string()];
        let list2: Vec<String> = vec!["Piatti".to_string(), "The Grill at Torrey Pines".to_string(), "Hungry Hunter Steakhouse".to_string(), "Shogun".to_string()]; 
        let result: Vec<String> = vec!["Shogun".to_string()];
        assert_eq!(Solution::find_restaurant(list1, list2), result);

        let list1: Vec<String> = vec!["Shogun".to_string(),"Tapioca Express".to_string(),"Burger King".to_string(),"KFC".to_string()];
        let list2: Vec<String> = vec!["KFC".to_string(),"Shogun".to_string(),"Burger King".to_string()]; 
        let result: Vec<String> = vec!["Shogun".to_string()];
        assert_eq!(Solution::find_restaurant(list1, list2), result);

        /*
        let list1: Vec<String> = vec!["Shogun".to_string(),"Tapioca Express".to_string(),"Burger King".to_string(),"KFC".to_string()];
        let list2: Vec<String> = vec!["KFC".to_string(),"Burger King".to_string(),"Tapioca Express".to_string(),"Shogun".to_string()]; 
        let result: Vec<String> = vec!["KFC".to_string(),"Burger King".to_string(),"Tapioca Express".to_string(),"Shogun".to_string()];
        assert_eq!(Solution::find_restaurant(list1, list2), result);
        */

        /*
        let list1: Vec<String> = vec!["Shogun".to_string(),"Tapioca Express".to_string(),"Burger King".to_string(),"KFC".to_string()];
        let list2: Vec<String> = vec!["KNN".to_string(),"KFC".to_string(),"Burger King".to_string(),"Tapioca Express".to_string(),"Shogun".to_string()]; 
        let result: Vec<String> = vec!["KFC".to_string(),"Burger King".to_string(),"Tapioca Express".to_string(),"Shogun".to_string()];
        assert_eq!(Solution::find_restaurant(list1, list2), result);
        */

        let list1: Vec<String> = vec!["KFC".to_string()];
        let list2: Vec<String> = vec!["KFC".to_string()]; 
        let result: Vec<String> = vec!["KFC".to_string()];
        assert_eq!(Solution::find_restaurant(list1, list2), result);
    }
}
