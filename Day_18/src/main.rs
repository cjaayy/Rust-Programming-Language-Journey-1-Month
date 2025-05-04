


// ❌ Example of a Dangling Reference (Rust won’t allow this):
// This will NOT compile!
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s // ❌ Trying to return a reference to local variable `s`
}

fn main() {
    let r = dangle(); // 💥 Compile-time error: `s` does not live long enough
    println!("{}", r);
}
*/

//🦀✅ Safe Version (return ownership instead):
fn no_dangle() -> String {
    let s = String::from("hello");
    s // ✅ Return the String itself (ownership is transferred)
}

fn main() {
    let r = no_dangle();
    println!("{}", r); // works fine!
}
