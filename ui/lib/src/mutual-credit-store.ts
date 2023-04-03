import { EntryHashB64, serializeHash } from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { writable, Writable } from 'svelte/store';

import { MutualCreditService } from './mutual-credit-service';
import { Offer, Transaction } from './types';

export class MutualCreditStore {
  private _offers: Writable<Record<EntryHashB64, Offer>> = writable({});

  private _transactions: Writable<Record<EntryHashB64, Transaction>> = writable(
    {}
  );

  constructor(
    protected cellClient: CellClient,
    public profilesStore: ProfilesStore
  ) {}

  get myAgentPubKey() {
    return serializeHash(this.cellClient.cellId[1]);
  }

  fetchMyPendingOffers() {
    
  }

  get myPendingOffers(): HoloHashed<Offer>[] {
    return Object.entries(this.offers)
      .filter(
        ([hash, offer]) =>
          !Object.values(this.transactions).find(t => t.offer_hash == hash)
      )
      .map(([hash, offer]) => ({
        hash,
        content: offer,
      }));
  }

  @computed
  get myTransactions(): HoloHashed<Transaction>[] {
    return Object.entries(this.transactions)
      .sort(
        ([_, transaction1], [__, transaction2]) =>
          transaction2.timestamp - transaction1.timestamp
      )
      .map(([hash, transaction]) => ({
        hash,
        content: transaction,
      }));
  }

  isOutgoing(multiparty: Multiparty): boolean {
    return multiparty.spender_pub_key === this.myAgentPubKey;
  }

  offer(offerHash: string): Offer {
    return this.offers[offerHash];
  }

  counterpartyKey(multiparty: Multiparty): string {
    return multiparty.recipient_pub_key === this.myAgentPubKey
      ? multiparty.spender_pub_key
      : multiparty.recipient_pub_key;
  }

  counterpartyNickname(multiparty: Multiparty): string {
    const counterpartyKey = this.counterpartyKey(multiparty);

    return this.profilesStore.profileOf(counterpartyKey).nickname;
  }

  @computed
  get outgoingOffers(): Array<HoloHashed<Offer>> {
    return this.myPendingOffers.filter(offer => this.isOutgoing(offer.content));
  }

  @computed
  get incomingOffers(): Array<HoloHashed<Offer>> {
    return this.myPendingOffers.filter(
      offer => !this.isOutgoing(offer.content)
    );
  }

  @computed
  get myBalance(): number {
    return Object.values(this.transactions).reduce(
      (acc, next) => acc + (this.isOutgoing(next) ? -next.amount : next.amount),
      0
    );
  }

  @action
  public async fetchMyPendingOffers() {
    const offers = await this.transactorService.queryMyPendingOffers();

    const promises = offers.map(o =>
      this.profilesStore.fetchAgentProfile(this.counterpartyKey(o.content))
    );
    await Promise.all(promises);

    offers.forEach(o => this.storeOffer(o));
  }

  @action
  public async fetchMyTransactions() {
    const transactions = await this.transactorService.getAgentTransactions(
      this.myAgentPubKey
    );

    const promises = transactions.map(t =>
      this.profilesStore.fetchAgentProfile(this.counterpartyKey(t.content))
    );
    await Promise.all(promises);

    transactions.forEach(t => this.storeTransaction(t));
  }

  @action
  public async createOffer(
    recipientPubKey: string,
    amount: number
  ): Promise<void> {
    await this.transactorService.createOffer(recipientPubKey, amount);

    this.fetchMyPendingOffers();
  }

  @action
  public async acceptOffer(offerHash: string): Promise<void> {
    await this.transactorService.acceptOffer(offerHash);

    runInAction(() => {
      this.fetchMyTransactions();
    });
  }

  @action
  public storeOffer(offer: HoloHashed<Offer>) {
    this.offers[offer.hash] = offer.content;
  }
  @action
  public storeTransaction(transaction: HoloHashed<Transaction>) {
    this.transactions[transaction.hash] = transaction.content;
  }
}
