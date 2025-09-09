fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 2;
    let b = 40;
    println!("{} + {} = {}", a, b, add(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 40), 42);
    }
}
