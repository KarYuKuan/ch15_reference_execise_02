fn main() {
    let mut s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    //下面的語句語法通過編譯，因為some_string是一個String的引用只是借用了s的值沒有取得所有權，所以無法對原值進行修改
    //some_string.push_str(", world");
}
