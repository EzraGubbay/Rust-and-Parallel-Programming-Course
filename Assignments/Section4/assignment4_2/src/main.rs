// Assignment 4.2: The Generic Callback System

struct Data<'a> {
    val: &'a str,
}

fn process_temp<F>(callback: F)
where
    F: for<'a> Fn(Data),
{
    let s = String::from("Temporary");
    let d = Data { val: &s };
    callback(d);
}

fn main() {
    process_temp(|d| println!("Callback saw: {}", d.val));
}
