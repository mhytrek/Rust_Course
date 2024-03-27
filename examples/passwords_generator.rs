use rand::{thread_rng, Rng};

fn generate_password(len: i32) -> String{
    if len == 0{
        return String::from("")
    }
    else{
        let mut rng = rand::thread_rng();
        let mut password: String = String::from("");
        for _ in 0..len{
            let num = rng.gen_range(0..10);
            password += &num.to_string()
        }
        return password;
    }
}


fn main() {
    let pass = generate_password(10);
    println!("{}", pass);
}