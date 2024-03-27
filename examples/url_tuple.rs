fn domain(url:&str) ->(usize, usize) {
    let mut start=0;
    let mut i=0;
    while i < url.len() -1 {
        if &url[i..i+2] == "//"{
            start = i + 2;
            i += 1
        }
        else if &url[i..i+1] == "/" {
            return (start, i)
        }
        i += 1
    }
    return (0,0)
}

fn path(url: &str) -> usize{
    for i in 0..url.len(){
        if &url[i..i+1] == "?"{
            return i
        }
    }
    return 0
}

fn find_tuple(url: &str) -> (&str, &str, &str) {
    let (d1, d2) = domain(&url);
    let p2 = path(&url[d2+1..]) + d2+1;
    println!("{}, {}, {}",d1,d2,p2);
    return (&url[d1..d2], &url[d2+1..p2], &url[p2+1..])
}


fn main(){
    let d = "https//www.domain.com/page?key1=value1&key2=value2";
    println!("{:?}", find_tuple(d));
}