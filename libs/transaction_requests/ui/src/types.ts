import { ActionHash, Record } from '@holochain/client';

export type TransactionRequestType = { Send: null } | { Receive: null };

export interface TransactionRequest {
  spender_pub_key: string;
  recipient_pub_key: string;

  amount: number;
}

export type TransactionRequestsSignal =
  | {
      type: 'TransactionRequestReceived';
      transaction_request: TransactionRequest;
    }
  | {
      type: 'TransactionRequestAccepted';
      transaction_request_hash: ActionHash;
      transaction: Record;
    }
  | {
      type: 'TransactionRequestCancelled';
      transaction_request_hash: ActionHash;
    }
  | {
      type: 'TransactionRequestRejected';
      transaction_request_hash: ActionHash;
    };
