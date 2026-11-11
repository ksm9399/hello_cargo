fn main() {
    // 불변 변수
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // 가변 변수
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {}", x);

    // 상수
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 섀도잉
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z);
    }
    println!("The value of z in the outer scope is: {}", z);

    // 섀도잉
    let spaces = "   ";     // 문자열 타입
    let spaces = spaces.len();      // 숫자 타입

    // mut = 변수의 타입을 바꿀 수 없음
    //let mut spaces = "   ";
    //spaces = spaces.len();


}
