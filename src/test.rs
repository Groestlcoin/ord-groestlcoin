pub(crate) use {
  super::*,
  crate::inscription::TransactionInscription,
  groestlcoin::blockdata::{opcodes, script},
  groestlcoin::Witness,
  pretty_assertions::assert_eq as pretty_assert_eq,
  std::iter,
  test_groestlcoincore_rpc::TransactionTemplate,
  unindent::Unindent,
};

macro_rules! assert_regex_match {
  ($value:expr, $pattern:expr $(,)?) => {
    let regex = Regex::new(&format!("^(?s){}$", $pattern)).unwrap();
    let string = $value.to_string();

    if !regex.is_match(string.as_ref()) {
      panic!(
        "Regex:\n\n{}\n\n…did not match string:\n\n{}",
        regex, string
      );
    }
  };
}

macro_rules! assert_matches {
  ($expression:expr, $( $pattern:pat_param )|+ $( if $guard:expr )? $(,)?) => {
    match $expression {
      $( $pattern )|+ $( if $guard )? => {}
      left => panic!(
        "assertion failed: (left ~= right)\n  left: `{:?}`\n right: `{}`",
        left,
        stringify!($($pattern)|+ $(if $guard)?)
      ),
    }
  }
}

pub(crate) fn blockhash(n: u64) -> BlockHash {
  let hex = format!("{n:x}");

  if hex.is_empty() || hex.len() > 1 {
    panic!();
  }

  hex.repeat(64).parse().unwrap()
}

pub(crate) fn txid(n: u64) -> Txid {
  let hex = format!("{n:x}");

  if hex.is_empty() || hex.len() > 1 {
    panic!();
  }

  hex.repeat(64).parse().unwrap()
}

pub(crate) fn outpoint(n: u64) -> OutPoint {
  format!("{}:{}", txid(n), n).parse().unwrap()
}

pub(crate) fn satpoint(n: u64, offset: u64) -> SatPoint {
  SatPoint {
    outpoint: outpoint(n),
    offset,
  }
}

pub(crate) fn address() -> Address {
  "grs1qw508d6qejxtdg4y5r3zarvary0c5xw7k3k4sj5"
    .parse()
    .unwrap()
}

pub(crate) fn recipient() -> Address {
  "tgrs1q6en7qjxgw4ev8xwx94pzdry6a6ky7wlfe9x8z6"
    .parse()
    .unwrap()
}

pub(crate) fn change(n: u64) -> Address {
  match n {
    0 => "tgrs1qjsv26lap3ffssj6hfy8mzn0lg5vte6a42h60lk",
    1 => "tgrs1qakxxzv9n7706kc3xdcycrtfv8cqv62hnwuzr78",
    2 => "tgrs1qxz9yk0td0yye009gt6ayn7jthz5p07a756cc4s",
    _ => panic!(),
  }
  .parse()
  .unwrap()
}

pub(crate) fn tx_in(previous_output: OutPoint) -> TxIn {
  TxIn {
    previous_output,
    script_sig: Script::new(),
    sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
    witness: Witness::new(),
  }
}

pub(crate) fn tx_out(value: u64, address: Address) -> TxOut {
  TxOut {
    value,
    script_pubkey: address.script_pubkey(),
  }
}

pub(crate) fn inscription(content_type: &str, body: impl AsRef<[u8]>) -> Inscription {
  Inscription::new(Some(content_type.into()), Some(body.as_ref().into()))
}

pub(crate) fn transaction_inscription(
  content_type: &str,
  body: impl AsRef<[u8]>,
  tx_in_index: u32,
  tx_in_offset: u32,
) -> TransactionInscription {
  TransactionInscription {
    inscription: inscription(content_type, body),
    tx_in_index,
    tx_in_offset,
  }
}

pub(crate) fn inscription_id(n: u32) -> InscriptionId {
  let hex = format!("{n:x}");

  if hex.is_empty() || hex.len() > 1 {
    panic!();
  }

  format!("{}i{n}", hex.repeat(64)).parse().unwrap()
}

pub(crate) fn envelope(payload: &[&[u8]]) -> Witness {
  let mut builder = script::Builder::new()
    .push_opcode(opcodes::OP_FALSE)
    .push_opcode(opcodes::all::OP_IF);

  for data in payload {
    builder = builder.push_slice(data);
  }

  let script = builder.push_opcode(opcodes::all::OP_ENDIF).into_script();

  Witness::from_vec(vec![script.into_bytes(), Vec::new()])
}
