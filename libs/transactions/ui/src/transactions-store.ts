import { asyncDerived, asyncReadable } from '@holochain-open-dev/stores';
import { CountersignedEntryRecord } from '@holochain-open-dev/utils';
import { TransactionsClient } from './transactions-client';
import { Transaction } from './types';
import { isOutgoing } from './utils';

export class TransactionsStore {
  constructor(public client: TransactionsClient) {}

  myTransactions = asyncReadable<Array<CountersignedEntryRecord<Transaction>>>(
    async set => {
      const transactions = await this.client.getAgentTransactions(
        this.client.appAgentClient.myPubKey
      );
      set(transactions);

      return this.client.onSignal(signal => {
        if (signal.type === 'NewTransactionCreated') {
          transactions.push(new CountersignedEntryRecord(signal.transaction));
          set(transactions);
        }
      });
    }
  );

  myBalance = asyncDerived(this.myTransactions, transactions =>
    transactions.reduce(
      (acc, t) =>
        acc +
        (isOutgoing(this.client.appAgentClient.myPubKey, t.entry)
          ? -t.entry.amount
          : t.entry.amount),
      0
    )
  );
}
