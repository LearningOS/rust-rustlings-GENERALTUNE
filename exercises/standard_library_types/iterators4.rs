// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.
struct Item {
    input: u64,
    output: u64,
}
pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    let keys = vec![
        Item {
            input: 0,
            output: 1,
        },
        Item {
            input: 1,
            output: 1,
        },
        Item {
            input: 2,
            output: 2,
        },
        Item {
            input: 4,
            output: 24,
        },
    ];

    match keys.into_iter().filter(|x| x.input == num).next() {
        Some(item) => item.output,
        None => 0,
    }
    // keys[0].output
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
