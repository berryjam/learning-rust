use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use hex;
use sha2::{Sha256, Sha512, Digest};

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
        for i in 0..next_pow_of_2 - data_len as i64 {
            new_data.push(b"".to_vec());
        }
        // Intertwine with randomness to obtain zero knowledge.
        let mut rand_list: Vec<Vec<u8>> = Vec::new();
        for i in 0..new_data.len() {
            let rand_string: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            rand_list.push(rand_string.into_bytes());
        }
        let mut res_data = Vec::new();
        for i in 0..data.len() {
            res_data.push(data.get(i).unwrap().clone());
            res_data.push(rand_list.get(i).unwrap().clone());
        }
        for i in 0..res_data.len() {
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
    let mut tree_node_id = value_id * 2 + i64::pow(base, (data_size as f64).log2().ceil() as u32);
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

pub fn test() {
    let zk_merkle_tree = ZkMerkleTree::new(vec![b"Yes".to_vec(), b"Sir".to_vec(), b"I Can".to_vec(), b"Boogie".to_vec()]);
    println!("{:?}", zk_merkle_tree.tree);
    println!("{}", zk_merkle_tree.get_root());
    let (val, auth_path) = zk_merkle_tree.get_val_and_path(1);
    println!("val:{} auth_path:{:?}", String::from_utf8(val.clone()).unwrap(), auth_path);
    println!("verify_zk_merkle_path res:{}", verify_zk_merkle_path(zk_merkle_tree.get_root(), zk_merkle_tree.data.len() as i64, 1, val, auth_path));
}