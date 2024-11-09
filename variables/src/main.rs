fn test_shadowing() {
    let i: i32 = 5;
    {
        let i = i + 5;
        println!("the variable i has the value {i} in this context due to shadowing.");
    }
    println!("the variable i has the value {i} in this context due to shadowing.");
}

fn test_assignment() {
    let i = 4;
    const Y: i32 = 1;
    let mut j = i + Y;
    j += 1;

    println!("the value of i = {i} and  Y = {Y} and j = {j}");
}

fn test_function(foo: i32) -> i32 {
    return foo + 1;
}

fn main() {
    test_shadowing();
    test_assignment();
    let i = 5;
    let i = test_function(i);
    println!("the value of i = {i}");
}
