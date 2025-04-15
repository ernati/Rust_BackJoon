fn is_palindrome( N : &String ) -> bool {
    let len = N.len();

    let mut v_N : Vec<char>= N.chars().collect();

    //A. 길이가 짝수라면,
    if len%2 == 0 {
        let mut start : usize = 0;
        let mut end : usize = len-1;

        let finish : usize = len / 2;

        //start가 finish와 동일해지면, loop를 종료한다.
        while start != finish {
            if v_N[start] != v_N[end] {return false;}
            
            start += 1;
            end -= 1;
        }

        return true;
    }
    else {
        let mut start : usize = 0;
        let mut end : usize = len-1;

        let finish : usize = len / 2 - 1;

        //start가 finish와 동일해지면, loop를 종료한다.
        while start != finish {
            if v_N[start] != v_N[end] {return false;}

            start += 1;
            end -= 1;
        }

        return true;
    }
}

fn main() {
    let mut v_i : Vec<usize> = vec![];
    let mut v_j : Vec<usize> = vec![];

    let mut answer : usize = 0;

    for i in 100..=999 {
        v_i.push(i);
        v_j.push(i);
    }

    for (index_i, item_i) in v_i.iter().enumerate() {
        for (index_j, item_j) in v_j.iter().enumerate() {
            //1. 이미 체크한 녀석들 ( index_i <= index_j )는 바로 넘어간다.
            if index_i <= index_j {continue;}

            //2. 두 수를 곱한 수를 문자열로 변환
            let number = item_i * item_j;
            let number_string = (number).to_string();

            //3. 대칭수인지 검사한다.
            if is_palindrome(&number_string) {
                if answer < number {
                    answer = item_i * item_j;
                }
            }
        }
    }

    println!("{answer}");
}

//1. 3자리 수 두개의 곱의 범위는 10000~999801