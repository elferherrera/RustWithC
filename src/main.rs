extern "C" {
    pub fn setSummand(val: i32);
    pub fn add(x: i32) -> i32;
}

fn main() {
    unsafe {
        setSummand(3);
        let x = add(2);
        println!("{}", x)
    }
}
