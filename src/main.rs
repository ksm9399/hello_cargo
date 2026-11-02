use std::io;
use rand::RngExt;
use std::cmp::Ordering;

fn main() {
    //println!("Hello, world!");
    println!("Guess the number!");

    // 임의 숫자 생성
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is : {secret_number}");

    // 반복문을 통해 여러 입력 받기
    loop {
        println!("Please input your guess. ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 문자열을 숫자로 변환
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {guess}");

        // 비밀번호와 추릿값 비교
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // 출력 테스트
    // let x = 5;
    // let y = 10;

    // println!("x = {x}, y = {}", y);
    // println!("x = {}, y = {}", x, y+2);
}
