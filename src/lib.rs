//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}

pub fn celsius_convert_lib(a: f32) -> f32 {
    let a = a / 5.0;
    let a = a * 9.0;
    a + 32.0
}

pub fn farenheit_convert_lib(a: f32)-> f32 {
    let a = a - 32.0;
    let a = a / 9.0;
    a * 5.0
}