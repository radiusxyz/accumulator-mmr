use std::{env};
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub use operator_mmr::{Error, Result, MMR, util::MemStore, MergeStringHash, StringHash};

#[derive(Debug)]
enum Mode {
  GenerateProof,
  VerifyProof
}

const PROOF: &str = "proof";
const VERIFY: &str = "verify";
const MAXIMUM_NODE: u64 = 10;

fn main() {

  let mode: Mode;
  let input: String;
  (mode, input) = get_arguments().expect("check your arguments");

  match mode {
    Mode::GenerateProof => generate_proof(input),
    Mode::VerifyProof => verify_proof(input)
  }
}

fn get_arguments() -> Result<(Mode, String)> {
  let args: Vec<String> = env::args().collect();

  if args.len() < 3 {
    return Err(Error::InvalidArgument);
  }

  let str_mode = args[1].clone();
  let input = args[2].clone();
  let enum_mode: Mode;

  if str_mode == PROOF {
    enum_mode = Mode::GenerateProof
  } else if str_mode == VERIFY {
    enum_mode = Mode::VerifyProof
  } else {
    return Err(Error::InvalidArgument);
  }

  return Ok((enum_mode, input));
}

fn generate_proof(input: String) {

  let file = fs::File::open("data/previous_state.txt");

  match file {
    Ok(_file) => append_and_get_proof(input),
    Err(_) => new_mmr_and_get_proof(input),
  }
}

// TODO : proof 생성해서 println!
fn append_and_get_proof(input: String) {

  let store = MemStore::default();
  let mut mmr = MMR::<_, MergeStringHash, _>::new(0, &store);

  let mut file = fs::OpenOptions::new().append(true).open("data/previous_state.txt").expect(
    "cannot open file");
  file.write_all(b"\n");
  file.write_all(input.as_bytes()).expect("Unable to write data");

  let file = fs::File::open("data/previous_state.txt").expect("Unable to read data");
  let reader = BufReader::new(file);

  let mut node_count = 0;
  for line in reader.lines() {
    let line = line.expect("Unable to read line");
    mmr.push(StringHash::from(line)).expect("push error");
    node_count += 1;
  }

  let proof = mmr.gen_proof(vec![node_count - 1]);
  println!("{{\"order\" : {}, \"proof\" : {:#?}}}", node_count, proof);

  if node_count >= MAXIMUM_NODE {
    fs::remove_file("data/previous_state.txt").expect("File delete error");
  }

}

fn new_mmr_and_get_proof(input: String) {

  let file = fs::File::create("data/previous_state.txt").expect("Unable to create file");
  let mut file = BufWriter::new(file);
  file.write_all(input.as_bytes()).expect("Unable to write data");

  let store = MemStore::default();
  let mut mmr = MMR::<_, MergeStringHash, _>::new(0, &store);
  mmr.push(StringHash::from(input)).expect("push error");
  let proof = mmr.gen_proof(vec![0]);
  println!("{{\"order\" : {}, \"proof\" : {:#?}}}", 1, proof);

}






// TODO : 검증 로직 찾기
fn verify_proof(_input: String) {
  // index 받아서 proof 검증
}