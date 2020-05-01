//#[cfg(test)]
mod tests {
    #[test]
    //fn it_works() {
    //    assert_eq!(2 + 2, 4);
    //}
//
//    #[areatest]
//    Run areatest
    fn area_works() {
        use crate::Rectangle;
        let recttest = Rectangle {
            width: 3,
            height: 5,
        };
        assert_eq!(15 u32, &recttest.area());
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn area_no_struct(width: u32, height: u32) -> u32 {
    width * height
}


pub fn area_struct(rectangle_dimensions: &Rectangle) -> u32 {
    rectangle_dimensions.width * rectangle_dimensions.height
}

pub fn perimeter(width: u32, height: u32) -> u32 {
    (width * 2) + ( height * 2)
}