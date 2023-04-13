import { asyncReadable } from '@holochain-open-dev/stores';
import { EntryRecord } from '@holochain-open-dev/utils';
import { TransactionRequestsClient } from './transaction-requests-client';
import { TransactionRequest } from './types';

export class TransactionRequestsStore {
  constructor(public client: TransactionRequestsClient) {}

  myTransactionRequests = asyncReadable<Array<EntryRecord<TransactionRequest>>>(
    async set => {
      let transactionRequests = await this.client.getMyTransactionRequests();
      set(transactionRequests);

      return this.client.onSignal(signal => {
        if (signal.type === 'TransactionRequestReceived') {
          transactionRequests.push(new EntryRecord(signal.transaction_request));
          set(transactionRequests);
        } else {
          transactionRequests = transactionRequests.filter(
            tr =>
              tr.actionHash.toString() !==
              signal.transaction_request_hash.toString()
          );
          set(transactionRequests);
        }
      });
    }
  );
}
