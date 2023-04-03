import { lazyLoadAndPoll, AsyncReadable } from "@holochain-open-dev/stores";
import { EntryRecord, LazyHoloHashMap } from "@holochain-open-dev/utils";
import { NewEntryAction, Record, ActionHash, EntryHash, AgentPubKey } from '@holochain/client';

import { TransactionRequestsClient } from './transaction-requests-client.js';

export class TransactionRequestsStore {

  constructor(public client: TransactionRequestsClient) {}
  
}
