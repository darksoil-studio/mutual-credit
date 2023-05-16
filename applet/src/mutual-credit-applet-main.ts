import { LitElement, css, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

import { localized, msg } from '@lit/localize';
import { hashState, sharedStyles } from '@holochain-open-dev/elements';
import '@darksoil/mutual-credit-transaction-requests/dist/elements/create-transaction-request.js';
import '@darksoil/mutual-credit-transactions/dist/elements/my-transaction-history.js';
import '@darksoil/mutual-credit-transactions/dist/elements/my-balance.js';
import '@darksoil/mutual-credit-transaction-requests/dist/elements/pending-transaction-requests.js';
import '@darksoil/mutual-credit-transaction-requests/dist/elements/transaction-request-detail.js';

import '@shoelace-style/shoelace/dist/components/button/button.js';

import { ActionHash } from '@holochain/client';
import SlTabGroup from '@shoelace-style/shoelace/dist/components/tab-group/tab-group.js';

type View =
  | { view: 'main' }
  | { view: 'create_transaction_request' }
  | { view: 'transaction_request_detail'; transactionRequestHash: ActionHash };

@localized()
@customElement('mutual-credit-applet-main')
export class MutualCreditAppletMain extends LitElement {
  @state(hashState())
  selectedTransactionRequest: ActionHash | undefined;

  @state() _view: View = { view: 'main' };

  renderCreateTransactionRequest() {
    return html` <div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <sl-button
            @click=${() => {
              this._view = { view: 'main' };
            }}
            style="position: absolute; left: 16px; top: 16px;"
            >${msg('Back')}</sl-button
          >
          <div class="column" style="flex: 1; align-items: center;">
            <create-transaction-request
              @transaction-request-created=${(e: CustomEvent) => {
                this._view = {
                  view: 'main',
                };
              }}
              style="margin-top: 16px; max-width: 600px"
            ></create-transaction-request>
          </div>
        </div>
      </div>
    </div>`;
  }

  renderTransactionRequestDetail(transactionRequestHash: ActionHash) {
    return html` <div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <sl-button
            @click=${() => {
              this._view = { view: 'main' };
            }}
            style="position: absolute; left: 16px; top: 16px;"
            >${msg('Back')}</sl-button
          >
          <div class="column" style="flex: 1; align-items: center;">
            <transaction-request-detail
              @transaction-completed=${(e: CustomEvent) => {
                this._view = {
                  view: 'main',
                };
              }}
              style="margin-top: 16px; max-width: 600px"
            ></transaction-request-detail>
          </div>
        </div>
      </div>
    </div>`;
  }

  render() {
    if (this._view.view === 'create_transaction_request')
      return this.renderCreateTransactionRequest();
    if (this._view.view === 'transaction_request_detail')
      return this.renderTransactionRequestDetail(
        this._view.transactionRequestHash
      );

    return html`
      <div class="column" style="align-items: center">
        <div class="row" style="margin-bottom: 16px">
          <div class="column">
            <span class="title" style="margin-bottom: 16px"
              >${msg('My Balance')}</span
            >
            <my-balance style="margin-right: 16px"></my-balance>
          </div>
          <div class="column">
            <span class="title" style="margin-bottom: 16px"
              >${msg('Transaction Requests')}</span
            >
            <pending-transaction-requests></pending-transaction-requests>
          </div>
        </div>
        <div class="column">
          <span class="title" style="margin-bottom: 16px"
            >${msg('Transaction History')}</span
          >
          <my-transaction-history></my-transaction-history>
        </div>
      </div>
    `;
  }

  static styles = [
    css`
      :host {
        display: flex;
        flex: 1;
      }
    `,
    sharedStyles,
  ];
}
