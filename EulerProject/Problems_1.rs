use std::io;
 
fn main() {
    //1. N 입력 받기
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("입력 실패");
    let N :usize = input.trim().parse().expect("유효한 숫자가 아닙니다.");
 
    //2. 3 또는 5의 배수를 모두 구하기
    let mut vAnswer : Vec<usize> = Vec::new();
 
    for i in 1..N { // 1 ~ N
        if i%3 == 0 { vAnswer.push(i); }
        else if i%5 == 0 { vAnswer.push(i); }
        else { /*Do Nothing*/ }
    }
    let mut answer : usize = 0;
    for i in &vAnswer {
        answer += i;
    }
 
    println!("{}", vAnswer.len());
    println!("{:?}", vAnswer);
    println!("Total Sum : {}", answer);
}
