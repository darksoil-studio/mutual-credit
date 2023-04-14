import {
  CountersignedEntryRecord,
  EntryRecord,
  ZomeClient,
} from '@holochain-open-dev/utils';
import {
  ActionHash,
  AgentPubKey,
  AppAgentClient,
  Delete,
  SignedActionHashed,
} from '@holochain/client';
import { Transaction } from '@darksoil/mutual-credit-transactions';

import {
  TransactionRequest,
  TransactionRequestsSignal,
  TransactionRequestType,
} from './types';

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
    return this.callZome('create_transaction_request', {
      transaction_request_type: transactionRequestType,
      counterparty_pub_key: counterpartyPublicKey,
      amount,
    });
  }

  async getTransactionRequestsForAgent(
    agent: AgentPubKey
  ): Promise<Array<ActionHash>> {
    return this.callZome('get_transaction_requests_for_agent', agent);
  }

  async getTransactionRequest(transactionRequestHash: ActionHash): Promise<
    | {
        transaction_request: EntryRecord<TransactionRequest>;
        deletes: Array<SignedActionHashed<Delete>>;
      }
    | undefined
  > {
    const details = await this.callZome(
      'get_transaction_request',
      transactionRequestHash
    );

    return {
      transaction_request: new EntryRecord(details.record),
      deletes: details.deletes,
    };
  }

  async getTransactionForTransactionRequest(
    transactionRequestHash: ActionHash
  ): Promise<ActionHash | undefined> {
    return this.callZome(
      'get_transaction_for_transaction_request',
      transactionRequestHash
    );
  }

  async acceptTransactionRequest(
    transactionRequestHash: ActionHash
  ): Promise<CountersignedEntryRecord<Transaction>> {
    return this.callZome('accept_transaction_request', transactionRequestHash);
  }

  async cancelTransactionRequest(
    transactionRequestHash: ActionHash
  ): Promise<void> {
    return this.callZome('cancel_transaction_request', transactionRequestHash);
  }

  async rejectTransactionRequest(
    transactionRequestHash: ActionHash
  ): Promise<void> {
    return this.callZome('reject_transaction_request', transactionRequestHash);
  }

  async clearTransactionRequests(
    transactionRequestsHashes: Array<ActionHash>
  ): Promise<void> {
    return this.callZome(
      'clear_transaction_requests',
      transactionRequestsHashes
    );
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
