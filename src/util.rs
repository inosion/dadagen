use rand::{thread_rng, Rng};
use std::ops::Range;

/**
 * Pull a random value from a comma-separated list of strings.
 *
 * @param comma_separated_list A comma-separated list of strings.
 * @return A randomly selected string from the list.
 */
fn random_from_string_separated_list(comma_separated_list: &str) -> String {
    let list: Vec<&str> = comma_separated_list.split(',').map(|s| s.trim()).collect();
    let index = thread_rng().gen_range(0..list.len());
    list[index].to_string()
}

/**
 * Generate a random integer between 1 and the specified upper bound (inclusive).
 *
 * @param upper The upper bound for the random integer.
 * @return A random integer.
 */
fn random_int_upto(upper: i32) -> i32 {
    thread_rng().gen_range(1..=upper)
}

/**
 * Generate a random integer within an inclusive range.
 *
 * @param min The minimum value of the range.
 * @param max The maximum value of the range.
 * @return A random integer within the specified range.
 */
fn random_int_range(min: i32, max: i32) -> i32 {
    thread_rng().gen_range(min..=max)
}

/**
 * Generate a random long integer within an inclusive range.
 *
 * @param min The minimum value of the range.
 * @param max The maximum value of the range.
 * @return A random long integer within the specified range.
 */
fn random_long_range(min: i64, max: i64) -> i64 {
    thread_rng().gen_range(min..=max)
}

/**
 * Generate a random double within a range with the specified number of decimal places.
 *
 * @param min The minimum value of the range.
 * @param max The maximum value of the range.
 * @param decimal_places The number of decimal places in the result.
 * @return A random double within the specified range.
 */
fn random_double_range(min: f64, max: f64, decimal_places: u32) -> f64 {
    let random_double: f64 = thread_rng().gen_range(min..max);
    (random_double * 10.0_f64.powi(decimal_places as i32)).round()
        / 10.0_f64.powi(decimal_places as i32)
}

/**
 * Pull a random value from a list of elements.
 *
 * @param alist A list of elements.
 * @return A randomly selected element from the list.
 */
fn random_from_list<E: Clone>(alist: &[E]) -> E {
    let index = thread_rng().gen_range(0..alist.len());
    alist[index].clone()
}

/**
 * Generate a random index using weighted probabilities.
 *
 * @param weights A list of weights for different options.
 * @return A random index based on the weighted probabilities.
 */
fn random_weighted_index(weights: &[f32]) -> usize {
    let sum: f32 = weights.iter().sum();
    let rnd: f32 = thread_rng().gen_range(0.0..1.0);
    let mut accumulated_weight = 0.0;
    for (i, weight) in weights.iter().enumerate() {
        accumulated_weight += *weight / sum;
        if rnd < accumulated_weight {
            return i;
        }
    }
    weights.len() - 1
}

/**
 * Pull a random value from a list by index.
 *
 * @param alist A list of elements.
 * @param index A list of integers representing positions in the main list.
 * @return A randomly selected element from the list by index.
 */
fn random_from_list_by_index<E: Clone>(alist: &[E], index: &[usize]) -> E {
    let random_index = thread_rng().gen_range(0..index.len());
    let selected_index = index[random_index];
    alist[selected_index].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_from_string_separated_list() {
        let result = random_from_string_separated_list("a, b, c");
        assert!(result == "a" || result == "b" || result == "c");
    }

    #[test]
    fn test_random_int_upto() {
        let result = random_int_upto(5);
        assert!(result >= 1 && result <= 5);
    }

    #[test]
    fn test_random_int_range() {
        let result = random_int_range(1, 5);
        assert!(result >= 1 && result <= 5);
    }

    #[test]
    fn test_random_long_range() {
        let result = random_long_range(1, 5);
        assert!(result >= 1 && result <= 5);
    }

    #[test]
    fn test_random_double_range() {
        let result = random_double_range(1.0, 5.0, 2);
        assert!(result >= 1.0 && result <= 5.0);
    }

    #[test]
    fn test_random_from_list() {
        let list = vec![1, 2, 3];
        let result = random_from_list(&list);
        assert!(list.contains(&result));
    }

    #[test]
    fn test_random_weighted_index_general() {
        let weights = vec![0.1, 0.2, 0.3, 0.4];
        let result = random_weighted_index(&weights);
        assert!(result >= 0 && result <= 3);
    }

    #[test]
    fn test_random_weighted_index_fixed() {
        let weights = vec![0.0, 1.0, 0.0, 0.0];
        let result = random_weighted_index(&weights);
        assert!(result == 1);
    }

    #[test]
    fn test_random_weighted_index_first() {
        let weights = vec![1.0, 0.0, 0.0, 0.0];
        let result = random_weighted_index(&weights);
        assert!(result == 0);
    }

    #[test]
    fn test_random_weighted_index_last() {
        let weights = vec![0.0, 0.0, 0.0, 1.0];
        let result = random_weighted_index(&weights);
        assert!(result == 3);
    }

    #[test]
    fn test_random_from_list_by_index() {
        let list = vec![1, 2, 3];
        let index = vec![0, 2];
        let result = random_from_list_by_index(&list, &index);
        assert!(result == 1 || result == 3);
    }
}
