import { lazyLoadAndPoll, AsyncReadable } from "@holochain-open-dev/stores";
import { EntryRecord, LazyHoloHashMap } from "@holochain-open-dev/utils";
import { NewEntryAction, Record, ActionHash, EntryHash, AgentPubKey } from '@holochain/client';

import { TransactionsClient } from './transactions-client.js';

export class TransactionsStore {

  constructor(public client: TransactionsClient) {}
  
}
