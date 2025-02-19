use super::*;

#[derive(Boilerplate)]
pub(crate) struct TransactionHtml {
  pub(crate) blockhash: Option<BlockHash>,
  pub(crate) chain: Chain,
  pub(crate) etching: Option<SpacedRune>,
  pub(crate) inscription_count: u32,
  pub(crate) transaction: Transaction,
  pub(crate) txid: Txid,
}

impl PageContent for TransactionHtml {
  fn title(&self) -> String {
    format!("Transaction {}", self.txid)
  }
}

#[cfg(test)]
mod tests {
  use {super::*, groestlcoin::blockdata::script};

  #[test]
  fn html() {
    let transaction = Transaction {
      version: 2,
      lock_time: LockTime::ZERO,
      input: vec![TxIn {
        sequence: Default::default(),
        previous_output: Default::default(),
        script_sig: Default::default(),
        witness: Default::default(),
      }],
      output: vec![
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(0).into_script(),
        },
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(1).into_script(),
        },
      ],
    };

    let txid = transaction.txid();

    pretty_assert_eq!(
      TransactionHtml {
        blockhash: None,
        chain: Chain::Mainnet,
        etching: None,
        inscription_count: 0,
        txid: transaction.txid(),
        transaction,
      }.to_string(),
      format!(
        "
        <h1>Transaction <span class=monospace>{txid}</span></h1>
        <dl>
        </dl>
        <h2>1 Input</h2>
        <ul>
          <li><a class=monospace href=/output/0000000000000000000000000000000000000000000000000000000000000000:4294967295>0000000000000000000000000000000000000000000000000000000000000000:4294967295</a></li>
        </ul>
        <h2>2 Outputs</h2>
        <ul class=monospace>
          <li>
            <a href=/output/{txid}:0 class=monospace>
              {txid}:0
            </a>
            <dl>
              <dt>value</dt><dd>0</dd>
              <dt>script pubkey</dt><dd class=monospace>OP_0</dd>
            </dl>
          </li>
          <li>
            <a href=/output/{txid}:1 class=monospace>
              {txid}:1
            </a>
            <dl>
              <dt>value</dt><dd>0</dd>
              <dt>script pubkey</dt><dd class=monospace>OP_PUSHNUM_1</dd>
            </dl>
          </li>
        </ul>
      "
      )
      .unindent()
    );
  }

  #[test]
  fn with_blockhash() {
    let transaction = Transaction {
      version: 2,
      lock_time: LockTime::ZERO,
      input: Vec::new(),
      output: vec![
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(0).into_script(),
        },
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(1).into_script(),
        },
      ],
    };

    assert_regex_match!(
      TransactionHtml {
        blockhash: Some(blockhash(0)),
        chain: Chain::Mainnet,
        etching: None,
        inscription_count: 0,
        txid: transaction.txid(),
        transaction,
      }
      .to_string(),
      "
        <h1>Transaction <span class=monospace>[[:xdigit:]]{64}</span></h1>
        <dl>
          <dt>block</dt>
          <dd><a href=/block/0{64} class=monospace>0{64}</a></dd>
        </dl>
        .*
      "
      .unindent()
    );
  }
}
