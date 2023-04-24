use std::sync::Arc;
use std::thread;

#[derive(Clone, Debug)]
enum Mod3<T = ()>
where
    T: Default,
{
    Een(T),
    Twee,
    Drie(Arc<i32>),
}

impl<T: Default> Mod3<T> {
    fn next(&mut self) {
        match *self {
            Mod3::Een(_) => *self = Mod3::Twee,
            Mod3::Twee => *self = Mod3::Drie(Arc::new(0)),
            Mod3::Drie(_) => *self = Mod3::Een(T::default()),
        }
    }
}

fn count(counter: &mut Mod3) {
    counter.next();
    println!("Counter is {:?}", counter);
}

fn main() {
    let mut counter = Mod3::Een(());
    count(&mut counter);
    let mut counter_clone = counter.clone();
    let handle2 = thread::spawn(move || count(&mut counter));
    let handle3 = thread::spawn(move || count(&mut counter_clone));
    let _ = handle2.join();
    let _ = handle3.join();
}
