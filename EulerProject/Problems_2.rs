use std::io;
 
fn main() {
    let mut answer : usize = 2;
 
    let mut vFibonacci : Vec<usize> = vec![1,1,2];
 
    //2. Fibonacci 수열 구하기
    loop {
        //1. 현재 vFibonacci 벡터의 마지막 두 원소 획득 
        let vLen : usize = vFibonacci.len();
        //2. 두값을 더한 새로운 마지막 원소 획득
        let newLastElement : usize = vFibonacci[ vFibonacci.len()-2 ] + vFibonacci[ vFibonacci.len()-1 ];
        if newLastElement%2 == 0 { answer += newLastElement; }
 
        //3. vFibonacci에 추가
        vFibonacci.push(newLastElement);
 
        //4. 마지막 항이 400만보다 커지면, 종료
        if newLastElement > 4000000 { break; }
    }
    println!("{}", answer);
}
