use std::io;
use std::collections::HashMap;

fn main() {
    //1. 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 실패");
    let N: usize = input.trim().parse().expect("유효한 숫자가 아닙니다");

    //2. 대화를 담을 HashMap 생성
    let mut HashMapConversation = HashMap::new();
    let mut answer = 0;

    //3. 대화 추적
    for _ in 0..N {
        //3.1 대화 담기
        input.clear();
        io::stdin().read_line(&mut input).expect("입력 실패");
        input = input.trim().to_string();
        
        //3.2 입력된 대화별로 분류
        //3.2.A ENTER - 지금까지의 HashMap의 <K,V> 갯수를 정답에 더하고, hashMap을 초기화함.
        if input == "ENTER" {
            answer += HashMapConversation.len();
            HashMapConversation = HashMap::new();
        }
        else  { // 3.2.B 나머지 - user 문자열이 나오면, 이미 user가 있다면 아무것도 하지않고 user가 없다면 삽입
            HashMapConversation.entry(input.clone()).or_insert(0);
        }
    }

    //4. 종료 후, 마지막으로 한번 더 answer에 더해준 후 출력한다.
    answer += HashMapConversation.len();

    println!("{}", answer);
}


// N : 채팅창의 기록 수
// 새로운 사람이 입장하면, ENTER가 입력된다.
// ENTER가 입력된 후, 각 유저의 최초의 발화는 무조건 곰곰 -> 이걸 세야 함.
// 그 다음 각 유저의 중복된 대화는 곰곰이 아님.
// HashMap?