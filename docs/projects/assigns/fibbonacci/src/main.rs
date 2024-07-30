use std::io; 

fn main() {
    let mut n = String::new();
    println!("Enter how far you want fib to go:");
    io::stdin().read_line(&mut n).expect("Couldnt read line");
    let n:u32 = match n.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("pls enter num"); 
            return
        }
    };
    if n==0 {
        println!("0");
        return
    } else if n==1 {
        println!("0\n1");
        return
    } else {
        println!("0\n1");
        let mut a = 0;
        let mut b = 1;
        for _ in 1..n{
            println!("{}", a+b);
            let temp = a+b;
            a = b;
            b= temp;
        }
    }
}
