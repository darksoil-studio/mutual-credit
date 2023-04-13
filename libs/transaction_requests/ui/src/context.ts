import { createContext } from '@lit-labs/context';

import { TransactionRequestsStore } from './transaction-requests-store.js';

export const transactionRequestsStoreContext =
  createContext<TransactionRequestsStore>(
    'hc_zome_mutual_credit_transaction_requests/store'
  );
