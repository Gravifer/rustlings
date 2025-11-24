#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// DONE: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    match b {
        0 => Err(DivisionError::DivideByZero),
        -1 if a == i64::MIN => Err(DivisionError::IntegerOverflow),
        _ if a % b != 0 => Err(DivisionError::NotDivisible),
        _ => Ok(a / b),
    }
}

// DONE: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result</*[i64; 4]*/Vec<i64>, DivisionError> { // * Vec<T> implements Deref<Target = [T]>
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // division_results.collect::<Result<Vec<i64>, DivisionError>>()
    //     .and_then(|vec| {
    //         if vec.len() == 4 {
    //             Ok([vec[0], vec[1], vec[2], vec[3]])
    //         } else {
    //             Err(DivisionError::NotDivisible) // This case should not happen in this context
    //         }
    //     })
    division_results.collect()
}

// DONE: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> Vec</*[*/Result<i64, DivisionError>/*; 4]*/> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // [
    //     division_results.clone().next().unwrap(),
    //     division_results.clone().nth(1).unwrap(),
    //     division_results.clone().nth(2).unwrap(),
    //     division_results.clone().nth(3).unwrap(),
    // ]
    division_results.collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
