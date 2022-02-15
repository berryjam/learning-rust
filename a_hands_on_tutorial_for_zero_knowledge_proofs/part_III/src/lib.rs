use std::borrow::Borrow;
use std::fmt::Debug;
use rand::{Rng};
use rand::distributions::Alphanumeric;
use rand_seeder::{Seeder};
use rand_pcg::Pcg64;
use hex;
use sha2::{Sha256, Digest};
use part_I::get_witness;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Proof {
    pub merkle_root: String,
    pub query_idx: i64,
    pub first_query_val: Vec<u8>,
    pub first_auth_path: Vec<String>,
    pub second_query_val: Vec<u8>,
    pub second_auth_path: Vec<String>,
}

// Well, implement Debug for solana_rbpf::vm::Executable in solana-rbpf...
impl Debug for Proof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Proof")
            .field("merkle_root", &self.merkle_root)
            .field("query_idx", &self.query_idx)
            .field("first_query_val", &self.first_query_val)
            .field("first_auth_path", &self.first_auth_path)
            .field("second_query_val", &self.second_query_val)
            .field("second_auth_path", &self.second_auth_path)
            .finish()
    }
}

pub fn hash_string(s: &str) -> String {
    let mut hasher = Sha256::new();
    // write input message
    hasher.update(s);
    // read hash digest and consume hasher
    let result = hasher.finalize();
    hex::encode(result.as_slice())
}

/*
A Zero Knowledge Merkle tree implementation using SHA256
 */
pub struct ZkMerkleTree {
    pub data: Vec<Vec<u8>>,
    pub tree: Vec<String>,
}

impl ZkMerkleTree {
    pub fn new(data: Vec<Vec<u8>>) -> Box<Self> {
        let mut new_data = data.clone();
        let mut tree = Vec::new();
        let base = 2;
        let data_len = new_data.len() as f64;
        let mut next_pow_of_2 = i64::pow(base, data_len.log2().ceil() as u32);
        if new_data.len() == 1 {
            next_pow_of_2 = 2
        }
        for _i in 0..next_pow_of_2 - data_len as i64 {
            new_data.push(b"".to_vec());
        }
        // Intertwine with randomness to obtain zero knowledge.
        let mut rand_list: Vec<Vec<u8>> = Vec::new();
        for _i in 0..new_data.len() {
            let rand_string: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            rand_list.push(rand_string.into_bytes());
        }
        let mut res_data = Vec::new();
        for i in 0..new_data.len() {
            res_data.push(new_data.get(i).unwrap().clone());
            res_data.push(rand_list.get(i).unwrap().clone());
        }
        for _i in 0..res_data.len() {
            tree.push("".to_string());
        }
        for i in 0..res_data.len() {
            tree.push(hash_string(
                String::from_utf8(res_data.get(i).unwrap().clone()).unwrap().as_str()));
        }
        for i in (1..res_data.len()).rev() {
            let parent_hash = format!("{}{}", tree.get(i * 2).unwrap(), tree.get(i * 2 + 1).unwrap());
            tree[i] = hash_string(parent_hash.as_str());
        }

        Box::new(Self {
            data: res_data,
            tree,
        })
    }

    pub fn get_root(&self) -> String {
        self.tree[1].clone()
    }

    pub fn get_val_and_path(&self, id: usize) -> (Vec<u8>, Vec<String>) {
        // Because of the zk padding, the data is now at id * 2
        let id = id * 2;
        let val = self.data[id].clone();
        let mut auth_path = Vec::new();
        let mut start_id = id + self.data.len();
        while start_id > 1 {
            auth_path.push(self.tree[start_id ^ 1].clone());
            start_id = start_id / 2;
        }

        (val, auth_path)
    }
}

pub fn verify_zk_merkle_path(root: String, data_size: i64, value_id: i64,
                             value: Vec<u8>, path: Vec<String>) -> bool {
    let mut cur = hash_string(String::from_utf8(value).unwrap().as_str());
    // Due to zk padding, data_size needs to be multiplied by 2, as does the value_id
    let base = 2;
    let mut tree_node_id = value_id * 2 + i64::pow(base, ((data_size * 2) as f64).log2().ceil() as u32);
    let iter = path.iter();
    for sibling in iter {
        assert_eq!(tree_node_id > 1, true);
        if tree_node_id % 2 == 0 {
            cur = hash_string(format!("{}{}", cur, sibling).as_str());
        } else {
            cur = hash_string(format!("{}{}", sibling, cur).as_str());
        }
        tree_node_id = tree_node_id / 2;
    }
    assert_eq!(tree_node_id == 1, true);

    root == cur
}

pub fn get_proof(problem: &Vec<i64>, assignment: &Vec<i64>, num_queries: i64) -> Vec<Proof> {
    let mut proofs = Vec::new();
    let mut randomness_seed = format!("{:?}", problem);
    for i in 0..num_queries {
        let witness = get_witness(problem.to_vec(), assignment.to_vec());
        let mut witness_data = Vec::new();
        for j in 0..witness.len() {
            witness_data.push(witness[j].to_be_bytes().to_vec());
        }
        let tree = ZkMerkleTree::new(witness_data);
        let mut rng: Pcg64 = Seeder::from(randomness_seed.as_str()).make_rng();
        let query_idx = rng.gen_range(0..problem.len() + 1);
        let (first_query_val, first_auth_path) = tree.get_val_and_path(query_idx);
        let (second_query_val, second_auth_path) = tree.get_val_and_path((query_idx + 1) % witness.len());
        let proof = Proof {
            merkle_root: tree.get_root(),
            query_idx: (query_idx as i64),
            first_query_val,
            first_auth_path,
            second_query_val,
            second_auth_path,
        };
        proofs.push(proof);
        randomness_seed += format!("{:?}", proofs[i as usize]).as_str();
    }

    proofs
}

pub fn verify_proof(problem: &Vec<i64>, proofs: Vec<Proof>) -> bool {
    let mut proof_checks_out = true;
    let mut randomness_seed = format!("{:?}", problem);
    for proof in proofs {
        let mut rng: Pcg64 = Seeder::from(randomness_seed.as_str()).make_rng();
        let query_idx = rng.gen_range(0..problem.len() + 1);
        proof_checks_out = proof_checks_out && (query_idx == proof.query_idx.try_into().unwrap());
        // Test witness properties.
        if query_idx < problem.len() {
            proof_checks_out = proof_checks_out && (
                (i64::from_be_bytes(proof.first_query_val.clone().try_into().unwrap()) - i64::from_be_bytes(proof.second_query_val.clone().try_into().unwrap())).abs() == problem[query_idx].abs());
        } else {
            proof_checks_out = proof_checks_out && (proof.first_query_val == proof.second_query_val);
        }
        // Authenticate paths
        proof_checks_out = proof_checks_out && (verify_zk_merkle_path(proof.merkle_root.clone(), (problem.len() + 1) as i64, query_idx as i64, proof.first_query_val.clone(), proof.first_auth_path.clone()));
        proof_checks_out = proof_checks_out && (verify_zk_merkle_path(proof.merkle_root.clone(), (problem.len() + 1) as i64, ((query_idx + 1) % (problem.len() + 1)) as i64, proof.second_query_val.clone(), proof.second_auth_path.clone()));
        randomness_seed += format!("{:?}", proof).as_str();
    }

    proof_checks_out
}

pub fn test() {
    let proof = Proof {
        merkle_root: "".to_string(),
        query_idx: 0,
        first_query_val: vec![],
        first_auth_path: vec![],
        second_query_val: vec![],
        second_auth_path: vec![],
    };
    println!("proof:{:?}", proof);
    let zk_merkle_tree = ZkMerkleTree::new(vec![b"Yes".to_vec(), b"Sir".to_vec(), b"I Can".to_vec(), b"Boogie".to_vec()]);
    println!("{:?}", zk_merkle_tree.tree);
    println!("{}", zk_merkle_tree.get_root());
    let (val, auth_path) = zk_merkle_tree.get_val_and_path(1);
    println!("val:{} auth_path:{:?}", String::from_utf8(val.clone()).unwrap(), auth_path);
    println!("verify_zk_merkle_path res:{}", verify_zk_merkle_path(zk_merkle_tree.get_root(), zk_merkle_tree.data.len() as i64, 1, val, auth_path));
}