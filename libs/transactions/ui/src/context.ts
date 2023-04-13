import { createContext } from '@lit-labs/context';
import { TransactionsStore } from './transactions-store.js';

export const transactionsStoreContext = createContext<TransactionsStore>(
  'hc_zome_mutual_credit_transactions/store_context'
);
