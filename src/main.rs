fn main() {
    let a = fizz_buzz(100);
    println!("{a:?}");
}

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();

    for number in 1..=n {
        let a = {
            if number % 3 == 0 && number % 5 == 0 {
                "FizzBuzz".to_string()
            } else if number % 3 == 0 {
                "Fizz".to_string()
            } else if number % 5 == 0 {
                "Buzz".to_string()
            } else {
                number.to_string()
            }
        };
        ans.push(a.to_string());
    }
    ans
}
