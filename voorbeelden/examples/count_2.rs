fn count(counter: &mut i32) -> &mut i32
{
    *counter = *counter + 1;
    println!("Counter is {counter}");
    counter
}

fn main()
{
    let mut counter = 0;
    let mut counter_ret = count(&mut counter);
    count(&mut counter);
    count(counter_ret);
}
