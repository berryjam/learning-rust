use rand::Rng;

pub fn get_witness(problem: Vec<i64>, assignment: Vec<i64>) -> Vec<i64> {
    /*
    Given an instance of a partition problem via a list of numbers (the problem) and a list of
    (-1, 1), we say that the assignment satisfies the problem if their dot product is 0.
     */
    let mut witness: Vec<i64> = Vec::new();
    let mut sum = 0;
    let mut mx = 0;
    let side_obfuscator = 1 - 2 * rand::thread_rng().gen_range(0..2);
    witness.push(sum);
    assert_eq!(problem.len(), assignment.len());
    for idx in 0..problem.len() {
        let num = *problem.get(idx).unwrap();
        let side = *assignment.get(idx).unwrap();
        assert_eq!(side == 1 || side == -1, true);
        sum += side * num * side_obfuscator;
        witness.push(sum);
        if num > mx {
            mx = num
        }
    }
    // make sure that it is a satisfying assignment
    assert_eq!(sum, 0);
    let shift = rand::thread_rng().gen_range(0..mx + 1);
    for idx in 0..witness.len() {
        witness[idx] = *witness.get(idx).unwrap() + shift;
    }

    witness
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_witness() {
        let problem = vec![4, 11, 8, 1];
        let assignment = vec![1, -1, 1, -1];
        let witness = get_witness(problem, assignment);
    }
}