fn count(counter: &mut i32) -> &mut i32
{
    *counter = *counter + 1;
    println!("Counter is {counter}");
    counter
}

fn main()
{
    let mut counter = 0;
    count(&mut counter);
    count(&mut counter);
    count(&mut counter);
}
