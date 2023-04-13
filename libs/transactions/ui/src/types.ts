import { AgentPubKey, Record } from '@holochain/client';

export interface Transaction {
  spender_pub_key: AgentPubKey;
  recipient_pub_key: AgentPubKey;

  amount: number;
  info: Uint8Array;
}

export type TransactionsSignal = {
  type: 'NewTransactionCreated';
  transaction: Record;
};
