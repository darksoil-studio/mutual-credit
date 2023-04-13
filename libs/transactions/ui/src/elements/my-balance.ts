import { sharedStyles } from '@holochain-open-dev/elements';
import { StoreSubscriber } from '@holochain-open-dev/stores';
import { consume } from '@lit-labs/context';
import { localized, msg } from '@lit/localize';
import { html, LitElement } from 'lit';
import { customElement } from 'lit/decorators.js';

import '@holochain-open-dev/elements/dist/elements/display-error.js';

import { transactionsStoreContext } from '../context.js';
import { TransactionsStore } from '../transactions-store.js';

@localized()
@customElement('my-balance')
export class MyBalance extends LitElement {
  /**
   * @internal
   */
  @consume({ context: transactionsStoreContext })
  transactionsStore!: TransactionsStore;

  /**
   * @internal
   */
  _myBalance = new StoreSubscriber(
    this,
    () => this.transactionsStore.myBalance,
    () => []
  );

  renderBalance(balance: number) {
    const roundedBalance = Math.round(balance * 100) / 100;
    return html`
      <div class="column center-content" style="flex: 1;">
        <span style="font-size: 24px; margin: 16px;">
          ${roundedBalance > 0 ? '+' : '-'}${roundedBalance} ${msg('credits')}
        </span>
      </div>
    `;
  }

  render() {
    switch (this._myBalance.value.status) {
      case 'pending':
        return html`
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
        `;
      case 'complete':
        return this.renderBalance(this._myBalance.value.value);
      case 'error':
        return html`
          <display-error
            tooltip
            .headline=${msg('Error fetching the balance')}
            .error=${this._myBalance.value.error.data.data}
          ></display-error>
        `;
    }
  }

  static get styles() {
    return sharedStyles;
  }
}
