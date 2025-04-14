use std::io;

fn main() {
    //1. N 입력 받기
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("입력 실패");
    let N :usize = input.trim().parse().expect("유효한 숫자가 아닙니다.");
    let mut tmp_N = N;
    let mut answer = 0;

    //2. N-1부터 2까지 체크
    for i in 2..=(N.isqrt()) {
        //2.A i가 2일 때는
        if i==2 {
            if tmp_N % 2 != 0 { continue; }

            //2.A.1 2로 나누어지지 않을 때까지 나눈다.
            tmp_N = loop {
                if tmp_N % i != 0 {break tmp_N} 
                
                tmp_N = tmp_N / i;
            };
            
            //2.A.2 2를 최대 소인수로 지정한다.
            answer = 2;
        }
        
        //2.B i가 3이상
        else {
            //2.B.1 짝수는 제외한다.
            if i%2==0 { continue; }

            //2.B.2 i로 나누어지지않는다면, skip한다.
            if tmp_N%i != 0 { continue; }

            //2.B.3 i로 나누어지지않을 때까지 나눈다.
            tmp_N = loop {
                if tmp_N % i != 0 {break tmp_N} 
                
                tmp_N = tmp_N / i;
            };

            //2.B.4 i를 최대 소인수로 지정한다.
            answer = i;
        }
    }

    println!("{}", answer);
}

// N의 소인수 중에서 가장 큰 수를 구하라.
// 1. 먼저 소수여야 하고
// 2. N의 인수여야 한다.

// 즉, N의 인수 중 소수를 구하면 된다.

// => 오류 수정
// 가장 큰 소인수를 구하는 것이므로, 인수 중 소수를 구하는 것과는 다르다.
// 예를 들어, 36의 가장 큰 소인수는 3이다. -> 인수 중 소수는 3이긴 하지만, 굳이 3 이후 수를 찾아낼 필요가 없다.

// 소인수를 구하는 방법
// 2부터 시작한다.
// 해당 수로 나누어 떨어지지 않을 만큼 수를 나눈다 (원본 수에서 해당 소인수를 완전히 제거 한다.)
// 3부터는 홀수만 진행한다.
