// 제네릭 데이터 타입
// 시그니처의 이름과 타입만 다르고 동작은 같은 두 함수 

/* 
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The most biggest number: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The most biggest char: {}", result);
}
*/

// 이 두 함수를 제네릭으로 만들어 코드 중복을 제거할 수 있는 코드를 만들면..

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The most biggest number: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The most biggest char: {}", result);
}

// PartialOrd는 Rust의 표준 라이브러리에 정의된 트레이트(trait) 중 하나입니다. 
// 이 트레이트는 비교 연산자(<, <=, >, >=)를 사용하여 값의 순서를 비교할 수 있도록 합니다.

// Copy 트레이트를 사용하는 이유는 간단하게 메모리 복사가 필요한 타입들에 대해 더 편리하게 작업할 수 있도록 
// 하기 위함입니다. 이로써 복사가 발생하는 경우에도 런타임 비용이 매우 낮아집니다.





