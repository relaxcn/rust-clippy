#![warn(clippy::dataflow_test)]



fn ff(s: &String) -> &String {
    ff1(s)
}

fn ff1(s: &String) -> &String {
    s
}

fn foo() {
    let s = String::from("hello");

    let s2 = &s;

    let _s3 = ff1(&s);
}

fn main() {

}
