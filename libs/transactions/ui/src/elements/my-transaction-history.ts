import { LitElement, html, PropertyValues } from 'lit';
import { customElement, property } from 'lit/decorators.js';

import '@shoelace-style/shoelace/dist/components/icon/icon.js';
import '@shoelace-style/shoelace/dist/components/spinner/spinner.js';
import { sharedStyles, wrapPathInSvg } from '@holochain-open-dev/elements';
import { localized, msg } from '@lit/localize';
import { transactionsStoreContext } from '../context';
import { consume } from '@lit-labs/context';
import {
  asyncDerived,
  asyncDeriveStore,
  StoreSubscriber,
} from '@holochain-open-dev/stores';
import { Transaction } from '../types';
import {
  AgentPubKeyMap,
  CountersignedEntryRecord,
  EntryRecord,
} from '@holochain-open-dev/utils';
import {
  Profile,
  ProfilesStore,
  profilesStoreContext,
} from '@holochain-open-dev/profiles';
import '@holochain-open-dev/elements/dist/elements/display-error.js';
import { mdiCallMade, mdiCallReceived } from '@mdi/js';

import { TransactionsStore } from '../transactions-store.js';
import { counterparty, isOutgoing } from '../utils.js';

@localized()
@customElement('my-transaction-history')
export class MyTransactionHistory extends LitElement {
  /**
   * @internal
   */
  @consume({ context: transactionsStoreContext })
  transactionsStore!: TransactionsStore;

  /**
   * @internal
   */
  @consume({ context: profilesStoreContext })
  profilesStore!: ProfilesStore;

  /**
   * @internal
   */
  _myTransactions = new StoreSubscriber(
    this,
    () =>
      asyncDeriveStore(this.transactionsStore.myTransactions, transactions => {
        const counterparties = transactions.map(t =>
          counterparty(
            this.transactionsStore.client.appAgentClient.myPubKey,
            t.entry
          )
        );

        return asyncDerived(
          this.profilesStore.agentsProfiles(counterparties),
          profiles =>
            [transactions, profiles] as [
              CountersignedEntryRecord<Transaction>[],
              AgentPubKeyMap<Profile>
            ]
        );
      }),
    () => []
  );

  renderTransactions(
    myTransactions: Array<EntryRecord<Transaction>>,
    profiles: AgentPubKeyMap<Profile>
  ) {
    if (myTransactions.length === 0)
      return html`<div class="padding">
        <span class="placeholder"
          >${msg('You have no transactions in your history.')}</span
        >
      </div>`;

    return html`
      <div class="column" style="flex: 1">
        ${myTransactions.map(
          (transaction, i) => html`
            <div class="row" style="align-items: center;">
              <sl-icon
                .style="color: ${isOutgoing(
                  this.transactionsStore.client.appAgentClient.myPubKey,
                  transaction.entry
                )
                  ? 'red'
                  : 'green'}"
                .src=${wrapPathInSvg(
                  isOutgoing(
                    this.transactionsStore.client.appAgentClient.myPubKey,
                    transaction.entry
                  )
                    ? mdiCallMade
                    : mdiCallReceived
                )}
              >
              </sl-icon>
              <span>
                ${isOutgoing(
                  this.transactionsStore.client.appAgentClient.myPubKey,
                  transaction.entry
                )
                  ? msg('To ')
                  : msg('From ')}
                ${profiles.get(
                  counterparty(
                    this.transactionsStore.client.appAgentClient.myPubKey,
                    transaction.entry
                  )
                ).nickname}
                <sl-relative-time
                  .date=${new Date(transaction.action.timestamp / 1000)}
                ></sl-relative-time>
              </span>
              <span style="font-size: 20px; margin: 0 24px;">
                ${isOutgoing(
                  this.transactionsStore.client.appAgentClient.myPubKey,
                  transaction.entry
                )
                  ? '-'
                  : '+'}${transaction.entry.amount}
                ${msg('credits')}
              </span>
            </div>

            ${i < myTransactions.length - 1
              ? html`<li divider padded role="separator"></li> `
              : html``}
          `
        )}
      </div>
    `;
  }

  render() {
    switch (this._myTransactions.value.status) {
      case 'pending':
        return html`
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
        `;
      case 'complete':
        return this.renderTransactions(
          this._myTransactions.value.value[0],
          this._myTransactions.value.value[1]
        );
      case 'error':
        return html`
          <display-error
            tooltip
            .headline=${msg('Error fetching the transaction history')}
            .error=${this._myTransactions.value.error.data.data}
          ></display-error>
        `;
    }
  }

  static styles = sharedStyles;
}
