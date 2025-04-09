use std::io;
use std::collections::HashMap;

fn main() {
    //1. N 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 실패");
    let N: usize = input.trim().parse().expect("유효한 숫자가 아닙니다");

    //2. HashMap 생성
    let mut MeetPeopleHashMap  = HashMap::new();

    //3. 대화
    for _ in 0..N {
        //3.1 대화 입력 받아서 공백으로 구분함. 
        input.clear();
        io::stdin().read_line(&mut input).expect("입력 실패");
    
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();
        
        let person1 : String = tokens[0].to_string();
        let person2 : String = tokens[1].to_string();

        let mut bIsPerson1Dancing : bool = false;
        let mut bCanSkipPerson2 : bool = false;
        let mut bIsPerson2Dancing : bool = false;

        //3.2 person1 검사
        //3.2.1 person1이 ChongChong인가?
        if person1 == "ChongChong"  {
            bIsPerson1Dancing = true;
            bIsPerson2Dancing = true;
            bCanSkipPerson2 = true;
        }
        //3.2.2 person1이 HashMap에 있는가?
        else if MeetPeopleHashMap.get(&person1).is_some() {  
            if let Some(dance) = MeetPeopleHashMap.get(&person1) { // person1이 HashMap에 존재하고,
                if *dance == true {                                // person1이 true라면( 춤을 추고 있다면 )
                    bIsPerson1Dancing = true;
                    bIsPerson2Dancing = true;
                    bCanSkipPerson2 = true;
                }
                else { // person1이 false라면,
                    //Do Nothing
                }
            }
        }
        else {  // ChongChong도 아니고, HashMap에도 없음 -> 무조건 false
            //Do Nothing
        }

        //3.3 person2 검사
        if bCanSkipPerson2 == false {
            if person2 == "ChongChong"  {
                bIsPerson1Dancing = true;
                bIsPerson2Dancing = true;
            }
            else if MeetPeopleHashMap.get(&person2).is_some() {  
                if let Some(dance) = MeetPeopleHashMap.get(&person2) { // person2이 HashMap에 존재하고,
                    if *dance == true {                                // person2이 true라면( 춤을 추고 있다면 )
                        bIsPerson1Dancing = true;
                        bIsPerson2Dancing = true;
                    }
                    else { // person2이 false라면,
                        //Do Nothing
                    }
                }
            }
            else {  // ChongChong도 아니고, HashMap에도 없음 -> 무조건 false
                //Do Nothing
            }
        }

        //3.4 검사 결과에 따라 HashMap에 삽입
        if bIsPerson1Dancing == true || bIsPerson2Dancing == true  {
            MeetPeopleHashMap.insert(person1, true);
            MeetPeopleHashMap.insert(person2, true);
        }
        else {
            MeetPeopleHashMap.insert(person1, false);
            MeetPeopleHashMap.insert(person2, false);
        }
    }

    //println!("{:#?}", MeetPeopleHashMap);

    //4. true인 사람들 수를 구하기
    let mut answer = 0;
    for (_,val) in &MeetPeopleHashMap {
        if *val == true {
            answer += 1;
        }
    }

    println!("{}", answer);


}