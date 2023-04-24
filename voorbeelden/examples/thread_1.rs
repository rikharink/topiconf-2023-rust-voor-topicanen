use std::rc::Rc;
use std::thread;

#[derive(Clone, Debug)]
enum Mod3<T = ()>
where
    T: Default,
{
    Een(T),
    Twee,
    Drie(Rc<i32>),
}

impl<T: Default> Mod3<T> {
    fn next(&mut self) {
        match *self {
            Mod3::Een(_) => *self = Mod3::Twee,
            Mod3::Twee => *self = Mod3::Drie(Rc::new(0)),
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
    count(&mut counter);
    thread::spawn(|| count(&mut counter));
}
