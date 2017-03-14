// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers
//#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>,
    // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    pub fn print_number(&self) {
        let mut s = String::new();
        for e in &self.data {
            s = format!("{}{}", s, e)
        }
        println!("{}", s)
    }

    pub fn print_min(&self) {
        if self.data.is_empty() {
            println!("No numbers!");
            return
        }
        let mut min: u64 = self.data[0];
        for &e in &self.data {
            min = if min < e { min } else { e }
        }
        println!("Smallest number is: {}", min)
    }

    pub fn print_max(&self) {
        if self.data.is_empty() {
            println!("No numbers!");
            return
        }
        let mut max: u64 = self.data[0];
        for &e in &self.data {
            max = if max < e { e } else { max }
        }
        println!("Biggest number is: {}", max)
    }

    // We can convert any vector of digits into a number, by removing trailing zeros. The `mut`
    // declaration for `v` here is just like the one in `let mut ...`: We completely own `v`, but Rust
    // still asks us to make our intention of modifying it explicit. This `mut` is *not* part of the
    // type of `from_vec` - the caller has to give up ownership of `v` anyway, so they don't care anymore
    // what you do to it.
    // 
    // **Exercise 05.1**: Implement this function.
    // 
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        if v.is_empty() { return BigInt::new(0) }
        while v[v.len() - 1] == 0 {
            let removed = v.pop();
            if removed.is_none() { break }
        }
        BigInt { data: v }
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0, 1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v);
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt { data: self.data.clone() }
    }
}

// We can also make the type `SomethingOrNothing<T>` implement `Clone`. 
use part02::{SomethingOrNothing, Something, Nothing};

impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            Something(ref element) => Something(element.clone())
        }
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the number of
// digits? The number of non-zero digits? The smallest/largest digit? Of course, these should all take `self` as a shared reference (i.e., in borrowed form).

pub fn main() {
    let b = BigInt::new(5);
    println!("{}", b.data[0]);

    let v = vec![1, 2, 3, 4, 5, 0, 0, 0];
    let vec = BigInt::from_vec(v);
    println!("{}", vec.data[0]);
    vec.print_number();
    vec.print_min();
    vec.print_max();
}

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}

fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 1337;
}

