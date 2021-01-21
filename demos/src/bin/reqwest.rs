use reqwest::blocking as rw;

fn main() {
    let body = rw::get("https://www.rust-lang.org")
        .unwrap()
        .text()
        .unwrap();
    println!("{}", body);
}
