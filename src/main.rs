use bytes::Bytes;
use hex;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{env, str};

pub use accumulator_mmr::helper::{get_peaks, leaf_index_to_mmr_size, leaf_index_to_pos};
pub use accumulator_mmr::{util::MemStore, Error, MergeStringHash, MerkleProof, Result, StringHash, MMR};

#[derive(Debug)]
enum Mode {
  GenerateProof,
  VerifyProof,
}

const PROOF: &str = "proof";
const VERIFY: &str = "verify";
const MAXIMUM_NODE: u64 = 1024;

fn main() {
  let mode: Mode;
  let file_name: String;
  let input: String;
  let args: Vec<String>;
  (mode, file_name, input, args) = get_arguments().expect("check your arguments");

  match mode {
    Mode::GenerateProof => generate_proof(input, file_name),
    Mode::VerifyProof => verify_proof(input, file_name, args),
  }
}

fn get_arguments() -> Result<(Mode, String, String, Vec<String>)> {
  let args: Vec<String> = env::args().collect();

  let str_mode = args[1].clone();
  let file_name = args[2].clone();
  let input = args[3].clone();
  let enum_mode: Mode;

  if str_mode == PROOF {
    enum_mode = Mode::GenerateProof
  } else if str_mode == VERIFY {
    enum_mode = Mode::VerifyProof
  } else {
    return Err(Error::InvalidArgument);
  }

  return Ok((enum_mode, file_name, input, args));
}

fn generate_proof(input: String, file_name: String) {
  let file = fs::File::open(format!("data/{}.txt", file_name));

  match file {
    Ok(_file) => append_and_get_proof(input, file_name),
    Err(_) => new_mmr_and_get_proof(input, file_name),
  }
}

fn append_and_get_proof(input: String, file_name: String) {
  let store = MemStore::default();
  let mut mmr = MMR::<_, MergeStringHash, _>::new(0, &store);

  let mut file = fs::OpenOptions::new().append(true).open(format!("data/{}.txt", file_name)).expect("cannot open file");
  file.write_all(b"\n").expect("write error");
  file.write_all(input.as_bytes()).expect("Unable to write data");

  let file = fs::File::open(format!("data/{}.txt", file_name)).expect("Unable to read data");
  let reader = BufReader::new(file);

  let mut node_count = 0;
  for line in reader.lines() {
    let line = line.expect("Unable to read line");
    mmr.push(StringHash::from(line)).expect("push error");
    node_count += 1;
  }

  let elem_hash = StringHash::from(input);
  let elem_hex = string_hash_to_hex(&elem_hash);
  let merkle_proof: MerkleProof<StringHash, MergeStringHash> = mmr.gen_proof(vec![leaf_index_to_pos(node_count - 1)]).unwrap();
  let mmr_size: u64 = merkle_proof.mmr_size();
  let hex_proof_vec: Vec<String> = merkle_proof.proof_items().iter().map(|hash| string_hash_to_hex(hash)).collect();

  print!("{{\"order\" : {}, \"mmr_size\": {}, \"proof\" : {:?}, \"hash\" : {:?}}}", node_count, mmr_size, hex_proof_vec, elem_hex);

  if node_count >= MAXIMUM_NODE {
    fs::remove_file(format!("data/{}.txt", file_name)).expect("File delete error");
  }
}

fn string_hash_to_hex(hash: &StringHash) -> String {
  let StringHash(raw_input) = hash;
  let hex_string = hex::encode(raw_input.as_ref());

  hex_string
}

fn hex_to_string_hash(hex_string: String) -> StringHash {
  let decoded = hex::decode(hex_string.clone()).unwrap();
  let string_hash = StringHash(Bytes::from(decoded).to_vec().into());

  string_hash
}

fn new_mmr_and_get_proof(input: String, file_name: String) {
  let file = fs::File::create(format!("data/{}.txt", file_name)).expect("Unable to create file");
  let mut file = BufWriter::new(file);
  file.write_all(input.as_bytes()).expect("Unable to write data");

  let store = MemStore::default();
  let mut mmr = MMR::<_, MergeStringHash, _>::new(0, &store);
  let elem_hash = StringHash::from(input);
  let elem_hex = string_hash_to_hex(&elem_hash);
  mmr.push(elem_hash).expect("push error");

  let merkle_proof = mmr.gen_proof(vec![0]).unwrap();
  let mmr_size: u64 = merkle_proof.mmr_size();
  let hex_proof_vec: Vec<String> = merkle_proof.proof_items().iter().map(|hash| string_hash_to_hex(hash)).collect();

  print!("{{\"order\" : {}, \"mmr_size\" : {:?},  \"proof\" : {:?}, \"hash\" : {:?}}}", 1, mmr_size, hex_proof_vec, elem_hex);
}

fn verify_proof(input: String, file_name: String, args: Vec<String>) {
  let order: u64;
  let proof: MerkleProof<StringHash, MergeStringHash>;
  let elem_hash: StringHash;
  (order, proof, elem_hash) = parse_verify_args(args);

  verify_partial_proof(order, file_name, proof, elem_hash);
}

fn parse_verify_args(args: Vec<String>) -> (u64, MerkleProof<StringHash, MergeStringHash>, StringHash) {
  let order: u64 = args[3].parse().unwrap();
  let proof_length: usize = args[4].parse().unwrap();
  let elem_hex = &args[5 + proof_length];
  let hash_proof_vec: Vec<StringHash> = args[5..5 + proof_length].iter().map(|hex| hex_to_string_hash(hex.to_string())).collect();
  let elem_hash = hex_to_string_hash(elem_hex.to_string());

  let proof: MerkleProof<StringHash, MergeStringHash> = MerkleProof::new(leaf_index_to_mmr_size(order - 1), hash_proof_vec);

  (order, proof, elem_hash)
}

fn verify_partial_proof(order: u64, file_name: String, proof: MerkleProof<StringHash, MergeStringHash>, elem_hash: StringHash) {
  let store = MemStore::default();
  let mut mmr = MMR::<_, MergeStringHash, _>::new(0, &store);

  let file = fs::File::open(format!("data/{}.txt", file_name)).expect("Unable to read data");
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line.expect("Unable to read line");
    mmr.push(StringHash::from(line)).expect("push error");
  }

  let partial_mmr_size = proof.mmr_size();

  let peaks = get_peaks(partial_mmr_size)
    .into_iter()
    .map(|peak_pos| mmr.mmr_batch().get_elem(peak_pos).and_then(|elem| Ok(elem.expect("invalid element"))))
    .collect::<Result<Vec<StringHash>>>()
    .expect("invalid peak");

  let root = mmr.bag_rhs_peaks(peaks).expect("invalid root").ok_or(Error::InconsistentStore).expect("invalid root");
  let leaf = vec![(leaf_index_to_pos(order - 1), elem_hash)];

  let result = proof.verify(root, leaf).unwrap();

  print!("{:?}", result);
  assert!(result);
}
