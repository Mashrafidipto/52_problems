fn main() {
    
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("Error");
    
    let mut num_t = match t.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) =>{
            println!("Enter a number please");
            0
        },
    };
    let mut vec_numbers = vec![] ;
    loop {

        let mut user_in = String::new();
        std::io::stdin().read_line(&mut user_in).expect("Error");

        vec_numbers.push(user_in.trim().parse::<i32>().expect("Error"));

        if num_t == 1 {
            break;
        }
        num_t -= 1;
    }


    for i in vec_numbers.iter() {
        if i % 2 != 0 {
            println!("Odd");
        }else {
            println!("Even")
        }
    }

}
