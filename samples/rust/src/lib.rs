pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn never_called() {
    println!("This function is never called");
}

pub fn looped(name: &str) {
    println!("Hello, {name}!");
}

pub fn called_once() {
    println!("This function is called once");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn test_never_called() {
        never_called();
    }

    #[test]
    fn test_looped() {
        for _ in 0..500 {
            looped("world");
        }
    }

    #[test]
    fn test_called_once() {
        called_once();
    }
}
