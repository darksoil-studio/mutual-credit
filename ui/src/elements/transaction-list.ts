import { LitElement, html, PropertyValues } from 'lit';
import { property } from 'lit/decorators.js';

import {
  Icon,
  ListItem,
  List,
  CircularProgress,
} from '@scoped-elements/material-web';

import { sharedStyles } from './utils/shared-styles';
import { dateString } from '../utils';
import { TransactorStore } from '../transactor.store';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';

export abstract class TransactionList extends ScopedElementsMixin(LitElement) {
  /** Public attributes */

  /** Private properties */

  @property({ type: Boolean })
  _loading = true;

  static styles = sharedStyles;

  async firstUpdated() {
    await this._deps.store.fetchMyTransactions();
    this._loading = false;
  }

  render() {
    return html`<div class="column center-content">
      ${this.renderContent()}
    </div>`;
  }

  renderContent() {
    if (this._loading)
      return html`
        <div class="padding center-content column">
          <mwc-circular-progress indeterminate></mwc-circular-progress>
          <span class="placeholder" style="margin-top: 18px;"
            >Fetching transaction history...</span
          >
        </div>
      `;

    const myTransactions = this._deps.store.myTransactions;

    if (myTransactions.length === 0)
      return html`<div class="padding">
        <span class="placeholder"
          >You have no transactions in your history</span
        >
      </div>`;

    return html`
      <mwc-list style="width: 100%;">
        ${myTransactions.map(
          (transaction, i) => html`
            <div class="row" style="align-items: center;">
              <mwc-list-item
                twoline
                noninteractive
                graphic="avatar"
                style="flex: 1;"
              >
                <span>
                  ${this._deps.store.isOutgoing(transaction.content)
                    ? 'To '
                    : 'From '}
                  ${this._deps.store.counterpartyNickname(transaction.content)}
                  on ${dateString(transaction.content.timestamp)}
                </span>
                <span slot="secondary"
                  >${this._deps.store.counterpartyKey(transaction.content)}
                </span>
                <mwc-icon
                  slot="graphic"
                  .style="color: ${this._deps.store.isOutgoing(
                    transaction.content
                  )
                    ? 'red'
                    : 'green'}"
                  >${this._deps.store.isOutgoing(transaction.content)
                    ? 'call_made'
                    : 'call_received'}</mwc-icon
                >
              </mwc-list-item>

              <span style="font-size: 20px; margin: 0 24px;">
                ${this._deps.store.isOutgoing(transaction.content)
                  ? '-'
                  : '+'}${transaction.content.amount}
                credits
              </span>
            </div>
            ${i < myTransactions.length - 1
              ? html`<li divider padded role="separator"></li> `
              : html``}
          `
        )}
      </mwc-list>
    `;
  }

  getScopedElements() {
    return {
      'mwc-circular-progress': CircularProgress,
      'mwc-icon': Icon,
      'mwc-list-item': ListItem,
      'mwc-list': List,
    };
  }
}
