import {
  CountersignedEntryRecord,
  ZomeClient,
} from '@holochain-open-dev/utils';
import { AgentPubKey, AppAgentClient, Record } from '@holochain/client';
import { Transaction, TransactionsSignal } from './types';

export class TransactionsClient extends ZomeClient<TransactionsSignal> {
  constructor(
    public appAgentClient: AppAgentClient,
    public roleName: string,
    public zomeName = 'transactions'
  ) {
    super(appAgentClient, roleName, zomeName);
  }

  async getAgentBalance(agentPubKey: AgentPubKey): Promise<number> {
    return this.callZome('get_balance_for_agent', agentPubKey);
  }

  async getAgentTransactions(
    agentPubKey: AgentPubKey
  ): Promise<Array<CountersignedEntryRecord<Transaction>>> {
    const transactions: Record[] = await this.callZome(
      'get_transactions_for_agent',
      agentPubKey
    );
    return transactions.map(r => new CountersignedEntryRecord(r));
  }

  async queryMyTransactions(): Promise<
    Array<CountersignedEntryRecord<Transaction>>
  > {
    const transactions: Record[] = await this.callZome(
      'query_my_transactions',
      null
    );
    return transactions.map(r => new CountersignedEntryRecord(r));
  }
}
