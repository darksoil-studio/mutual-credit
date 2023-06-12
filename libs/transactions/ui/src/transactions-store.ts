import { asyncDerived, asyncReadable } from '@holochain-open-dev/stores';
import {
  CountersignedEntryRecord,
  LazyHoloHashMap,
} from '@holochain-open-dev/utils';
import { AgentPubKey } from '@holochain/client';
import { TransactionsClient } from './transactions-client';
import { Transaction } from './types';
import { isOutgoing } from './utils';

export class TransactionsStore {
  constructor(public client: TransactionsClient) {}

  transactionsForAgent = new LazyHoloHashMap((agent: AgentPubKey) =>
    asyncReadable<Array<CountersignedEntryRecord<Transaction>>>(async set => {
      const transactions = await this.client.getAgentTransactions(agent);
      set(transactions);

      return this.client.onSignal(signal => {
        if (signal.type === 'NewTransactionCreated') {
          transactions.push(new CountersignedEntryRecord(signal.transaction));
          set(transactions);
        }
      });
    })
  );

  myTransactions = this.transactionsForAgent.get(
    this.client.appAgentClient.myPubKey
  );

  balanceForAgent = new LazyHoloHashMap((agent: AgentPubKey) =>
    asyncDerived(this.transactionsForAgent.get(agent), transactions =>
      transactions.reduce(
        (acc, t) =>
          acc + (isOutgoing(agent, t.entry) ? -t.entry.amount : t.entry.amount),
        0
      )
    )
  );

  myBalance = this.balanceForAgent.get(this.client.appAgentClient.myPubKey);
}
