import { EntryRecord, ZomeClient } from '@holochain-open-dev/utils';
import { ActionHash, AgentPubKey, AppAgentClient } from '@holochain/client';
import { Transaction } from '@darksoil/mutual-credit-transactions';
import { TransactionRequestsSignal, TransactionRequestType } from './types';

export class TransactionRequestsClient extends ZomeClient<TransactionRequestsSignal> {
  constructor(
    public appAgentClient: AppAgentClient,
    public roleName: string,
    public zomeName = 'transaction_requests'
  ) {
    super(appAgentClient, roleName, zomeName);
  }

  async createTransactionRequest(
    transactionRequestType: TransactionRequestType,
    counterpartyPublicKey: AgentPubKey,
    amount: number
  ): Promise<string> {
    return this.callZome('create_transaction', {
      transaction_request_type: transactionRequestType,
      coutnerparty_pub_key: counterpartyPublicKey,
      amount,
    });
  }

  async acceptTransactionRequest(
    transactionRequestHash: ActionHash
  ): Promise<EntryRecord<Transaction>> {
    return this.callZome('accept_transaction_request', transactionRequestHash);
  }
  /* 
  async cancelOffer(offerHash: string) {
    await this.callZome('cancel_offer', {
      offer_hash: offerHash,
    });
  }

  async rejectOffer(offerHash: string) {
    await this.callZome('reject_offer', {
      offer_hash: offerHash,
    });
  } */
}
