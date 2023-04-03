import { createContext } from '@lit-labs/context';
import { TransactionRequestsStore } from './transaction-requests-store';

export const transactionRequestsStoreContext = createContext<TransactionRequestsStore>(
  'hc_zome_transaction_requests/store'
);

