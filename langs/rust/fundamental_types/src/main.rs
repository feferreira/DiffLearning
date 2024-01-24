
/*
fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}*/

/* types inference
fn build_vector_elide() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}*/



fn main() {
    //cast from types using as operator
    assert_eq!( 10_i8 as u16, 10_u16);
    assert_eq!( 2525_u16 as i16, 2525_i16);

    //out of range, truncated
    assert_eq!( -1_i8 as u8, 255_u8);
    assert_eq!( 255_u8 as i8, -1_i8);

    //std library provides operations as methods for ints
    assert_eq!( 2_u16.pow(4), 16);

    /* overflow
    let mut i = 1;
    loop {
	i *= 10; //panic in debug mode, wraps to negative in release 
    } */

    // checked operations return an option of the result
    assert_eq!(10_u8.checked_add(20), Some(30)); //10 + 30 can be an u8?
    assert_eq!(100_u8.checked_add(200), None); //100 + 200 cant

    // panic if overflows
    let x : u8 = 10;
    let y : u8 = 20;
    let _sum = x.checked_add(y).unwrap();

    //wrapping
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392); // 250000 modulo 2^16

    //saturating return result clamped
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    //overflowing return a tuple with a bool indicating wheter an overflow occurred
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
    
    //bool types
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    //chars are UTF-8
    assert_eq!('*' as i32, 42);

    //methods characters
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

    //tuples, ex:. fn split_at(&self, mid: usize) -> (&str, &str);
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    /*equivalent
    let text = "I see the eigenvalue in thine eye";
    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
    */
    println!("Hello, world!");
}
