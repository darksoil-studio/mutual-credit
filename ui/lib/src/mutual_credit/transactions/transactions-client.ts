import { 
  AppAgentClient, 
  Record, 
  ActionHash, 
  EntryHash, 
  AgentPubKey,
} from '@holochain/client';
import { isSignalFromCellWithRole, EntryRecord, ZomeClient } from '@holochain-open-dev/utils';

import { TransactionsSignal } from './types.js';

export class TransactionsClient extends ZomeClient<TransactionsSignal> {

  constructor(public client: AppAgentClient, public roleName: string, public zomeName = 'transactions') {
    super(client, roleName, zomeName);
  }
}
