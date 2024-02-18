use std::collections::HashMap;
use std::io;

fn main() {
    // 부서에 대한 직원 이름을 저장할 해시 맵 생성
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        // 사용자로부터 명령어 입력 받기
        let mut input = String::new();
        println!("명령어를 입력하세요 (예: add sally to engineering / list engineering):");
        io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

        // 입력 받은 명령어를 공백으로 분리하여 처리
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        // 명령어 해석
        if tokens.len() >= 4 && tokens[0] == "add" && tokens[2] == "to" {
            let employee_name = tokens[1].to_string();
            let department_name = tokens[3].to_string();
            add_employee(&mut departments, &employee_name, &department_name);
        } else if tokens.len() >= 2 && tokens[0] == "list" {
            let department_name = tokens[1].to_string();
            list_employees(&departments, &department_name);
        } else {
            println!("올바른 명령어 형식이 아닙니다.");
        }
    }
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>, employee_name: &str, department_name: &str) {
    let department = departments.entry(department_name.to_string()).or_insert(Vec::new());
    department.push(employee_name.to_string());
    println!("{}이(가) {} 부서에 추가되었습니다.", employee_name, department_name);
}

fn list_employees(departments: &HashMap<String, Vec<String>>, department_name: &str) {
    match departments.get(department_name) {
        Some(employees) => {
            println!("{} 부서 직원 목록:", department_name);
            for employee in employees {
                println!("{}", employee);
            }
        },
        None => {
            println!("{} 부서에는 직원이 없습니다.", department_name);
        }
    }
}
