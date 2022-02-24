use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("숫자를 맞춰라!");
    println!("원하는 숫자를 입력해주세요");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("올바른 값이 나오지 않았습니다");

        // 형변환 and 매치 열것값 에 따른 분기 설정
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("올바른 숫자를 입력해주세요");
                continue;
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("입력하신 숫자가 작습니다"),
            Ordering::Greater => println!("입력하신 숫자가 큽니다"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}