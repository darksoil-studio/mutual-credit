import { LitElement, css, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';

import { localized, msg } from '@lit/localize';
import { hashState, sharedStyles } from '@holochain-open-dev/elements';
import '@darksoil/mutual-credit-transaction-requests/dist/elements/create-transaction-request.js';
import '@darksoil/mutual-credit-transactions/dist/elements/my-transaction-history.js';
import '@darksoil/mutual-credit-transactions/dist/elements/my-balance.js';
import '@darksoil/mutual-credit-transaction-requests/dist/elements/pending-transaction-requests.js';
import '@darksoil/mutual-credit-transaction-requests/dist/elements/transaction-request-detail.js';

import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/tab/tab.js';
import '@shoelace-style/shoelace/dist/components/tab-group/tab-group.js';
import '@shoelace-style/shoelace/dist/components/tab-panel/tab-panel.js';

import { ActionHash } from '@holochain/client';

type View =
  | { view: 'main' }
  | { view: 'create_transaction_request' }
  | { view: 'transaction_request_detail'; transactionRequestHash: ActionHash };

@localized()
@customElement('applet-main')
export class AppletMain extends LitElement {
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
          <div
            class="column"
            style="flex: 1; align-items: center; margin: 16px"
          >
            <sl-card>
              <span slot="header">${msg('Transaction Request')}</span>
              <transaction-request-detail
                .transactionRequestHash=${transactionRequestHash}
                @transaction-completed=${(e: CustomEvent) => {
                  this._view = {
                    view: 'main',
                  };
                }}
                @transaction-request-rejected=${(e: CustomEvent) => {
                  this._view = {
                    view: 'main',
                  };
                }}
                @transaction-request-cancelled=${(e: CustomEvent) => {
                  this._view = {
                    view: 'main',
                  };
                }}
              ></transaction-request-detail>
            </sl-card>
          </div>
        </div>
      </div>
    </div>`;
  }

  render() {
    return html`
      ${this._view.view === 'create_transaction_request'
        ? this.renderCreateTransactionRequest()
        : html``}
      ${this._view.view === 'transaction_request_detail'
        ? this.renderTransactionRequestDetail(this._view.transactionRequestHash)
        : html``}
      <sl-tab-group
        id="tabs"
        placement="start"
        style=${styleMap({
          display: this._view.view === 'main' ? 'flex' : 'none',
          flex: '1',
        })}
      >
        <sl-tab slot="nav" panel="my_balance">${msg('My Balance')}</sl-tab>
        <sl-tab slot="nav" panel="transaction_requests"
          >${msg('Transaction Requests')}</sl-tab
        >

        <sl-tab-panel name="my_balance">
          <div class="flex-scrollable-parent">
            <div class="flex-scrollable-container">
              <div class="flex-scrollable-y">
                <div class="column" style="align-items: center">
                  <div class="column" style="width: 700px">
                    <my-balance
                      style="margin-bottom: 16px; margin-top: 64px"
                    ></my-balance>
                    <sl-card>
                      <span slot="header" class="title"
                        >${msg('Transaction History')}</span
                      >
                      <my-transaction-history></my-transaction-history>
                    </sl-card>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </sl-tab-panel>
        <sl-tab-panel name="transaction_requests">
          <div class="flex-scrollable-parent">
            <div class="flex-scrollable-container">
              <div class="flex-scrollable-y">
                <div class="column" style="align-items: center">
                  <sl-card style="margin-top: 16px">
                    <span slot="header" class="title"
                      >${msg('Transaction Requests')}</span
                    >
                    <pending-transaction-requests
                      @transaction-request-selected=${(e: CustomEvent) => {
                        this._view = {
                          view: 'transaction_request_detail',
                          transactionRequestHash:
                            e.detail.transactionRequestHash,
                        };
                      }}
                    ></pending-transaction-requests>
                  </sl-card>
                </div>
              </div>
            </div>
          </div>
          <sl-button
            variant="primary"
            @click=${() => {
              this._view = { view: 'create_transaction_request' };
            }}
            style="position: fixed; right: 16px; bottom: 16px"
          >
            ${msg('Create Transaction Request')}
          </sl-button>
        </sl-tab-panel>
      </sl-tab-group>
    `;
  }

  static styles = [
    css`
      :host {
        display: flex;
        flex: 1;
      }
      sl-tab-group::part(base) {
        display: flex;
        flex: 1;
      }
      sl-tab-group::part(body) {
        display: flex;
        flex: 1;
      }
      sl-tab-panel::part(base) {
        width: 100%;
        height: 100%;
      }
      .flex-scrollable-parent {
        width: 100%;
        height: 100%;
      }
      sl-tab-panel {
        width: 100%;
      }
      .tab-content {
        max-width: 900px;
        min-width: 700px;
      }
    `,
    sharedStyles,
  ];
}
