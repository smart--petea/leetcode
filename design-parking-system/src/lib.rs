struct ParkingSystem (i32, i32, i32); //big, medium, small


//1 - big car
//2 - medium car
//3 - small car
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
       Self (big, medium, small)
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => {
                if self.0 == 0 {
                    return false;
                }

                self.0 -= 1;
                true
            }
            2 => {
                if self.1 == 0 {
                    return false;
                }

                self.1 -= 1;
                true
            }
            3 => {
                if self.2 == 0 {
                    return false;
                }

                self.2 -= 1;
                true
            }
            _ => panic!("not existent car_type {}", car_type)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ps = ParkingSystem::new(1, 1, 0);
        assert_eq!(true, ps.add_car(1));
        assert_eq!(true, ps.add_car(2));
        assert_eq!(false, ps.add_car(3));
        assert_eq!(false, ps.add_car(1));
    }
}
