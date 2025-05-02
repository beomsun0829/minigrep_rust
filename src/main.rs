use std::env;
use std::fs;

/*
1. main 함수가 두가지 일을 수행함, 프로그램 증가에 따라 main 함수에서 처리하는 개별작업의 개수가 증가
2. main이 길어질수록 필요한 변수들이 더 많이 스코프 안에 있게 되고, 추적이 어려워짐
3. 파일 읽기 실패 시 expect를 사용했는데, 파일읽기실패가 다양한 경우가 있음에도 하나의 오류메세지만 출력함
4. 모든 에러처리코드가 한곳에 모여있는것이 추후 유지보수하기 좋을것
*/

fn main(){
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    println!("With text :\n{contents}");
}