import { createContext } from '@lit-labs/context';
import { TransactionsStore } from './transactions-store';

export const transactionsStoreContext = createContext<TransactionsStore>(
  'hc_zome_transactions/store'
);

