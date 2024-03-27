fn domain(url:&str) -> String{
    let mut start=0;
    let mut i=0;
    while i < url.len() -1 {
        if &url[i..i+2] == "//"{
            start = i + 2;
            i += 1
        }
        else if &url[i..i+1] == "/" {
            return String::from(&url[start..i])
        }
        i += 1
    }
    return "".to_string()
}


fn main() {
    let d = domain("https://kpietak.github.io/rust-course/02-ownership/05-exercises/");
    println!("{}", d);
}