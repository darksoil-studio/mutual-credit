import { 
  AppAgentClient, 
  Record, 
  ActionHash, 
  EntryHash, 
  AgentPubKey,
} from '@holochain/client';
import { isSignalFromCellWithRole, EntryRecord, ZomeClient } from '@holochain-open-dev/utils';

import { TransactionRequestsSignal } from './types.js';

export class TransactionRequestsClient extends ZomeClient<TransactionRequestsSignal> {

  constructor(public client: AppAgentClient, public roleName: string, public zomeName = 'transaction_requests') {
    super(client, roleName, zomeName);
  }
}
