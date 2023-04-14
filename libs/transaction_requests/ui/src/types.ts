import { EntryRecord } from '@holochain-open-dev/utils';
import { ActionHash, AgentPubKey, Record } from '@holochain/client';

export type TransactionRequestType = { Send: null } | { Receive: null };

export interface TransactionRequest {
  spender_pub_key: AgentPubKey;
  recipient_pub_key: AgentPubKey;

  amount: number;
}

export type TransactionRequestStatus =
  | 'pending'
  | 'completed'
  | 'cancelled'
  | 'rejected';

export interface TransactionRequestWithStatus {
  transactionRequest: EntryRecord<TransactionRequest>;
  status: TransactionRequestStatus;
}

export type TransactionRequestsSignal =
  | {
      type: 'TransactionRequestCreated';
      transaction_request: Record;
    }
  | {
      type: 'TransactionCompleted';
      transaction_request_hash: ActionHash;
      transaction: Record;
    }
  | {
      type: 'TransactionRequestCancelled';
      transaction_request_hash: ActionHash;
    }
  | {
      type: 'TransactionRequestCleared';
      transaction_request_hash: ActionHash;
    }
  | {
      type: 'TransactionRequestRejected';
      transaction_request_hash: ActionHash;
    };
