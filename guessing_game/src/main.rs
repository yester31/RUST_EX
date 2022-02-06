use std::io; // input/output 기능을 포함하는 표준 라이브러리

fn main() {//프로그램의 진입점
    println!("Guess the number!"); // string을 화면에 표시하는 매크로

    println!("Please input your guess.");

    // let foo = 5; // immutable
    // let mut bar = 5; // mutable 가변변수 선언
    // String UTF-8 인코딩의 문자열 타입
    // new 함수는 새로운 빈 String을 생성
    let mut guess = String::new(); // 입력값을 저장할 공간을 생성

    //read_line은 사용자가 표준 입력에 입력할 때마다 입력된 문자들을 하나의 문자열에 저장
    //&는 코드의 여러 부분에서 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자 
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // {}는 변경자로써 값이 표시되는 위치를 나타냄
    println!("You guessed: {}", guess);
}