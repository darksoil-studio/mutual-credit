import { asyncReadable } from '@holochain-open-dev/stores';
import { EntryRecord, LazyHoloHashMap } from '@holochain-open-dev/utils';
import { ActionHash, AgentPubKey } from '@holochain/client';
import { TransactionRequestsClient } from './transaction-requests-client';
import {
  TransactionRequest,
  TransactionRequestStatus,
  TransactionRequestWithStatus,
} from './types';

export function counterparty(
  myPubKey: AgentPubKey,
  transactionRequest: TransactionRequest
): AgentPubKey {
  if (transactionRequest.recipient_pub_key.toString() === myPubKey.toString())
    return transactionRequest.spender_pub_key;
  return transactionRequest.recipient_pub_key;
}

export function isOutgoing(
  myPubKey: AgentPubKey,
  transactionRequest: TransactionRequest
): boolean {
  return transactionRequest.recipient_pub_key.toString() === myPubKey.toString()
    ? true
    : false;
}

export class TransactionRequestsStore {
  constructor(public client: TransactionRequestsClient) {}

  transactionRequests = new LazyHoloHashMap(
    (transactionRequestHash: ActionHash) =>
      asyncReadable<TransactionRequestWithStatus | undefined>(async set => {
        let transactionRequestDetails = await this.client.getTransactionRequest(
          transactionRequestHash
        );
        if (!transactionRequestDetails) {
          set(undefined);
          return;
        }
        let transactionRequest = transactionRequestDetails.transaction_request;
        let status: TransactionRequestStatus = 'pending';
        if (transactionRequestDetails.deletes.length > 0) {
          const deleteAuthor =
            transactionRequestDetails.deletes[0].hashed.content.author;

          if (
            deleteAuthor.toString() ===
            this.client.appAgentClient.myPubKey.toString()
          ) {
            status = 'cancelled';
          } else {
            status = 'rejected';
          }
        }

        set({ transactionRequest, status });

        return this.client.onSignal(signal => {
          if (
            signal.type === 'TransactionRequestReceived' ||
            signal.transaction_request_hash.toString() !==
              transactionRequestHash.toString()
          )
            return;
          if (signal.type === 'TransactionCompleted') {
            set({
              transactionRequest: transactionRequest!,
              status: 'completed',
            });
          }
          if (signal.type === 'TransactionRequestCancelled') {
            set({
              transactionRequest: transactionRequest!,
              status: 'cancelled',
            });
          }
          if (signal.type === 'TransactionRequestRejected') {
            set({
              transactionRequest: transactionRequest!,
              status: 'rejected',
            });
          }
        });
      })
  );

  myTransactionRequests = asyncReadable<Array<ActionHash>>(async set => {
    let transactionRequests = await this.client.getTransactionRequestsForAgent(
      this.client.appAgentClient.myPubKey
    );
    set(transactionRequests);

    return this.client.onSignal(signal => {
      if (signal.type === 'TransactionRequestReceived') {
        transactionRequests.push(
          signal.transaction_request.signed_action.hashed.hash
        );
        set(transactionRequests);
      } else if (signal.type === 'TransactionRequestCleared') {
        transactionRequests = transactionRequests.filter(
          tr => tr.toString() !== signal.transaction_request_hash.toString()
        );
        set(transactionRequests);
      }
    });
  });
}
