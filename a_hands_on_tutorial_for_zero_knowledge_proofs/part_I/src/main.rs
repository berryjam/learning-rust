use part_I::get_witness;

fn main() {
    let problem = vec![4, 11, 8, 1];
    let assignment = vec![1, -1, 1, -1];
    let witness = get_witness(problem, assignment);
    println!("{:?}", witness);
}