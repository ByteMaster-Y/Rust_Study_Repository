// use 키워드로 경로를 현재 범위로 가져오면 경로의 아이템이 마치 현재 범위의 아이템인 것처럼 호출할 수 있다.
/* If you bring a path to the current scope with the use keyword, 
the items in the path can be called as if they were items in the current scope. */

mod front_of_house;

use front_of_house::hosting;

fn main() {
    eat_at_restaurant();
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::serve_food();
}

mod serving {
    pub fn serve_food() {
        println!("Food served");
    }
}
