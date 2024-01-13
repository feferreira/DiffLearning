//project generated with cargo new (a library or executable is called crate)
//build and exec with cargo run
use std::str::FromStr; //bring FromStr trait to scope, trait is a collection of methods that types can implement
use std::env; //use env module, to access args
//to view doc: rustup doc --std

//greatest common divisor
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
	if m < n {
	    let t = m; //could be let t: u64 = n;
	    m = n;
	    n = t;
	}
	m = m % n;
    }
    n //function return, not followed by a semicolon
}
//mut: var is mutable
//u64: var type, unsigned 64
//-> u64: return type
//assert! -> ! marks a macro invocation, assert is always checked
//let: declare a local var, rust can infer type


fn main() {
    let mut numbers = Vec::new(); //Vec<u64> rust infer

    for arg in env::args().skip(1) { //iterator, skip 1st args that program name
	numbers.push(u64::from_str(&arg) //use trait
		     .expect("error parsing arg"))
    }

    if numbers.len() == 0 {
	eprintln!("gcd NUMBER..."); //write to stderr
	std::process::exit(1);
    }

    let mut d = numbers[0]; //save first number
    //numbers own the vector, rust auto frees it when goes out of scope
    for m in &numbers[1..] { //borrow a reference to the vector elements
	d = gcd(d, *m); //dereference m, yielding the value m reference to
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
    //if main returns, is success, error should be returned explicity on expect or exit 
	
}
 

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

//#[test] is an attribute, like ifdef
//cargo test to run
