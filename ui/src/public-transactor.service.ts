import { HoloHashed, timestampToMillis } from '@holochain-open-dev/core-types';
import { AppWebsocket, CellId } from '@holochain/conductor-api';
import { Offer, Transaction } from './types';

export class PublicTransactorService {
  constructor(
    public appWebsocket: AppWebsocket,
    public cellId: CellId,
    public zomeName = 'transactor'
  ) {}

  async getMyPublicKey(): Promise<string> {
    return this.callZome('who_am_i', null);
  }

  async getAgentBalance(agentPubKey: string): Promise<number> {
    return this.callZome('get_balance_for_agent', agentPubKey);
  }

  async getAgentTransactions(
    agentPubKey: string
  ): Promise<Array<HoloHashed<Transaction>>> {
    const transactions = await this.callZome(
      'get_transactions_for_agent',
      agentPubKey
    );
    return transactions.map((t: any) => ({
      hash: t.hash,
      content: {
        ...t.content,
        timestamp: timestampToMillis(t.content.timestamp),
      },
    }));
  }

  async queryMyPendingOffers(): Promise<Array<HoloHashed<Offer>>> {
    return this.callZome('query_my_pending_offers', null);
  }

  async createOffer(recipientPubKey: string, amount: number): Promise<string> {
    return this.callZome('create_offer', {
      recipient_pub_key: recipientPubKey,
      amount,
    });
  }

  async acceptOffer(offerHash: string): Promise<string> {
    return this.callZome('accept_offer', offerHash);
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

  private callZome(fn_name: string, payload: any) {
    return this.appWebsocket.callZome({
      cap: null as any,
      cell_id: this.cellId,
      zome_name: this.zomeName,
      fn_name,
      payload,
      provenance: this.cellId[1],
    });
  }
}
