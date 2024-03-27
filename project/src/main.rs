fn hello_world() -> String{
    let result = String::from("Hello, World!");
    result
}
fn hello_name(name: String) -> String{
    let result = format!("Hello, {}!", name);
    result
}
fn main() {
    let greeting = hello_world();
    println!("{}", greeting);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_test() {
        let want = String::from("Hello, World!");
        let result = hello_world();
        assert_eq!(want, result);
    }

    #[test]
    fn hello_name_test() {
        let want = String::from("Hello, Rusty!");
        let result = hello_name(String::from("Rusty"));
        assert_eq!(want, result);
    }
}
