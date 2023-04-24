fn count(counter: &i32) -> &i32
{
        *counter = *counter + 1;
            println!("Counter is {counter}");
                counter
}

fn main()
{
        let counter = 0;
            let counter_ret = count(&counter);
                count(&counter);
                    count(counter_ret);
}
