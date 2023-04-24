use std::thread;

#[derive(Clone, Debug)]
struct Mod3 {
    value: i32,
}

impl Mod3 {
    fn next(&mut self) {
        self.value = (self.value % 3) + 1;
    }
}

fn count(counter: &mut Mod3) {
    counter.next();
    println!("Counter is {:?}", counter);
}

fn main() {
    let mut counter = Mod3 { value: 1 };
    count(&mut counter);
    count(&mut counter);
    let _ = thread::spawn(move || count(&mut counter)).join();
}
