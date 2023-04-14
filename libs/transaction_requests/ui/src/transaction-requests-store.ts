import {
  Transaction,
  TransactionsClient,
} from '@darksoil/mutual-credit-transactions';
import {
  asyncDeriveStore,
  asyncReadable,
  retryUntilSuccess,
} from '@holochain-open-dev/stores';
import {
  CountersignedEntryRecord,
  EntryRecord,
  LazyHoloHashMap,
} from '@holochain-open-dev/utils';
import { ActionHash, AgentPubKey } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
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
  return transactionRequest.spender_pub_key.toString() === myPubKey.toString()
    ? true
    : false;
}

export class TransactionRequestsStore {
  constructor(
    public client: TransactionRequestsClient,
    public transactionsClient: TransactionsClient
  ) {}

  transactionRequests = new LazyHoloHashMap(
    (transactionRequestHash: ActionHash) =>
      asyncDeriveStore(
        retryUntilSuccess(() =>
          this.client.getTransactionRequest(transactionRequestHash)
        ),
        transactionRequestDetails =>
          asyncReadable<TransactionRequestWithStatus | undefined>(async set => {
            let transactionHash =
              await this.client.getTransactionForTransactionRequest(
                transactionRequestHash
              );

            if (!transactionRequestDetails) {
              set(undefined);
              return;
            }
            let transactionRequest =
              transactionRequestDetails.transaction_request;
            let status: TransactionRequestStatus = 'pending';
            if (transactionHash) {
              status = 'completed';
            } else if (transactionRequestDetails.deletes.length > 0) {
              const deleteAuthor =
                transactionRequestDetails.deletes[0].hashed.content.author;

              if (
                deleteAuthor.toString() ===
                transactionRequest.action.author.toString()
              ) {
                status = 'cancelled';
              } else {
                status = 'rejected';
              }
            }

            set({ transactionRequest, status });

            const unsubscribe1 = this.client.onSignal(signal => {
              if (
                signal.type === 'TransactionRequestCreated' ||
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
            const unsubscribe2 = this.transactionsClient.onSignal(signal => {
              if (signal.type === 'NewTransactionCreated') {
                const transaction = new CountersignedEntryRecord<Transaction>(
                  signal.transaction
                );
                const transactionRequestHashInfo = decode(
                  transaction.entry.info
                ) as ActionHash;

                if (
                  transactionRequestHashInfo.toString() ===
                  transactionRequestHash.toString()
                ) {
                  set({
                    transactionRequest: transactionRequest!,
                    status: 'completed',
                  });
                }
              }
            });
            return () => {
              unsubscribe1();
              unsubscribe2();
            };
          })
      )
  );

  myTransactionRequests = asyncReadable<Array<ActionHash>>(async set => {
    let transactionRequests = await this.client.getTransactionRequestsForAgent(
      this.client.appAgentClient.myPubKey
    );
    set(transactionRequests);

    return this.client.onSignal(signal => {
      if (signal.type === 'TransactionRequestCreated') {
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
