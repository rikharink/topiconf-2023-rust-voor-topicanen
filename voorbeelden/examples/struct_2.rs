use std::thread;

#[derive(Clone, Debug)]
struct Mod3 {
    value: i32,
}

trait Optellen {
    type Input: Copy;

    fn optellen(&mut self, input: Self::Input);
}

impl Optellen for Mod3 {
    type Input = ();

    fn optellen(&mut self, _input: Self::Input) {
        self.value = (self.value % 3) + 1;
    }
}

fn count(counter: &mut Mod3) {
    counter.optellen(());
    println!("Counter is {:?}", counter);
}

fn main() {
    let mut counter = Mod3 { value: 1 };
    count(&mut counter);
    count(&mut counter);
    let _ = thread::spawn(move || count(&mut counter)).join();
}
