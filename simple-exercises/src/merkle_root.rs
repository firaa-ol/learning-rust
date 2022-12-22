use crate::error::{Error, Result};
use serde::Deserialize;
use sha2::Digest;
use std::fs::OpenOptions;

// only retrieve those fields we need
#[derive(Deserialize, Debug)]
struct Block {
    mrkl_root: String,
    tx: Vec<Transaction>,
}

#[derive(Deserialize, Debug)]
struct Transaction {
    hash: String,
}

pub fn compute_merkle_root() {
    match get_block_data() {
        Ok(block) => {
            let mut tx_data: Vec<Vec<u8>> = block
                .tx
                .iter()
                .map(|t| hex::decode(t.hash.to_owned()).unwrap())
                .collect();

            // https://bitcoindev.network/calculating-the-merkle-root-for-a-block/
            // Big Endian to Little Endian ; for Block with only coinbase transaction merkle root will be the same as the coinbase transaction
            if tx_data.len() > 1 {
                tx_data.iter_mut().for_each(|item| item.reverse());

                let calculated_root = get_merkle_root(&mut tx_data);
                println!("Block Merkle Root: {}", block.mrkl_root);
                println!("Calculated Merkle Root: {}", calculated_root);
                println!("Same : {}", block.mrkl_root == calculated_root);
            } else {
                println!("There is Only a Coinbase Transaction");
            }
        }

        Err(err) => println!("Error retrieving bitcoin block data {}", err),
    }
}

fn get_block_data() -> Result<Block> {
    let file_name = "bitcoin_block_200000.json";
    println!(
        "=====Calculating Merkle Root for {} =====",
        file_name.split('.').next().unwrap_or("")
    );
    let file = OpenOptions::new().read(true).open(file_name)?;
    match serde_json::from_reader(file) {
        Ok(block) => Ok(block),
        Err(err) => Err(Error::from(err)),
    }
}

fn get_merkle_root(txs: &mut Vec<Vec<u8>>) -> String {
    if txs.len() == 1 {
        // change it back to Big Endian
        txs[0].reverse();
        return hex::encode(&txs[0]);
    }

    if txs.len() % 2 != 0 {
        txs.push((&txs[txs.len() - 1]).to_owned());
    }

    let mut secondary: Vec<Vec<u8>> = vec![];
    for i in (0..txs.len()).step_by(2) {
        let mut data: Vec<u8> = vec![];
        data.extend(&txs[i]);
        data.extend(&txs[i + 1]);

        //bitcoin 2 times hashing
        let hash = hash256(&data);
        let hash = hash256(&hash);

        secondary.push(hash);
    }

    get_merkle_root(&mut secondary)
}

fn hash256(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(&data);
    hasher.finalize().to_vec()
}
