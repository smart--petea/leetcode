struct ParkingSystem (i32, i32, i32); //big, medium, small


//1 - big car
//2 - medium car
//3 - small car
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
       Self (big, medium, small)
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        let field_reference = match car_type {
            1 => &mut self.0,
            2 => &mut self.1,
            3 => &mut self.2,
            _ => panic!("")
        };

        Self::try_decrement(field_reference)
    }

    fn try_decrement(field: &mut i32) -> bool {
        match field {
            0 => false,
            _ => {
                *field -= 1;
                true
            }
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
