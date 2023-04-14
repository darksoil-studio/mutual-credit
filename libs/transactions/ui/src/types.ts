import { ActionHash, AgentPubKey, Record } from '@holochain/client';

export interface TransactionParty {
  agent_pub_key: AgentPubKey;
  previous_transaction_hash: ActionHash | undefined;
  resulting_balance: number;
}

export interface Transaction {
  spender: TransactionParty;
  recipient: TransactionParty;

  amount: number;
  info: Uint8Array;
}

export type TransactionsSignal = {
  type: 'NewTransactionCreated';
  transaction: Record;
};
