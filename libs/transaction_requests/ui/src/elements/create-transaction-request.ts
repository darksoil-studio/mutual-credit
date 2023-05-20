import { html, LitElement } from 'lit';
import { customElement, property, query, state } from 'lit/decorators.js';

import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/input/input.js';
import '@shoelace-style/shoelace/dist/components/card/card.js';
import '@shoelace-style/shoelace/dist/components/dialog/dialog.js';
import '@shoelace-style/shoelace/dist/components/alert/alert.js';
import '@shoelace-style/shoelace/dist/components/select/select.js';
import '@shoelace-style/shoelace/dist/components/option/option.js';
import '@holochain-open-dev/profiles/dist/elements/search-agent.js';

import {
  notifyError,
  onSubmit,
  sharedStyles,
} from '@holochain-open-dev/elements';
import { localized, msg } from '@lit/localize';
import SlDialog from '@shoelace-style/shoelace/dist/components/dialog/dialog.js';
import { AgentPubKey } from '@holochain/client';
import { consume } from '@lit-labs/context';

import { transactionRequestsStoreContext } from '../context.js';
import { TransactionRequestsStore } from '../transaction-requests-store.js';

@localized()
@customElement('create-transaction-request')
export class CreateTransactionRequest extends LitElement {
  @consume({ context: transactionRequestsStoreContext, subscribe: true })
  transactionRequestsStore!: TransactionRequestsStore;

  @query('#dialog')
  _dialog!: SlDialog;

  @state()
  transactionRequestInput:
    | {
        counterparty: AgentPubKey;
        amount: number;
        transactionRequestType: 'send' | 'receive';
      }
    | undefined;

  @state()
  creating = false;

  async createTransactionRequest() {
    if (this.creating) return;

    this.creating = true;
    try {
      await this.transactionRequestsStore.client.createTransactionRequest(
        this.transactionRequestInput!.transactionRequestType === 'send'
          ? { Send: null }
          : { Receive: null },
        this.transactionRequestInput!.counterparty,
        this.transactionRequestInput!.amount
      );

      this.dispatchEvent(
        new CustomEvent('transaction-request-created', {
          detail: {
            recipientPubKey: this.transactionRequestInput!.counterparty,
            amount: this.transactionRequestInput!.amount,
          },
          composed: true,
          bubbles: true,
        })
      );
      this._dialog.hide();
      this.transactionRequestInput = undefined;
    } catch (e) {
      notifyError(msg('Error creating the transaction request'));
      console.error(e);
    }
    this.creating = false;
  }

  renderConfirmDialog() {
    return html`
      <sl-dialog
        id="dialog"
        .label=${msg('Confirm Transaction Request')}
        @sl-request-close=${(e: CustomEvent) => {
          if (this.creating) {
            e.preventDefault();
          }
        }}
      >
        ${this.transactionRequestInput
          ? html`
              <span>
                You are about to create an transaction request to
                <agent-avatar
                  .agentPubKey=${this.transactionRequestInput.counterparty}
                ></agent-avatar
                >. This would
                ${this.transactionRequestInput.transactionRequestType === 'send'
                  ? 'lower'
                  : 'raise'}
                your balance by ${this.transactionRequestInput.amount} and
                ${this.transactionRequestInput.transactionRequestType === 'send'
                  ? 'raise'
                  : 'lower'}
                the agent receiving the request by the same amount.
              </span>
            `
          : html``}

        <sl-button
          slot="footer"
          @click=${() => {
            this._dialog.hide();
            this.transactionRequestInput = undefined;
          }}
        >
          ${msg('Cancel')}
        </sl-button>
        <sl-button
          slot="footer"
          variant="primary"
          @click=${() => this.createTransactionRequest()}
          .loading=${this.creating}
        >
          ${msg('Confirm')}
        </sl-button>
      </sl-dialog>
    `;
  }

  render() {
    return html`
      ${this.renderConfirmDialog()}
      <sl-card>
        <span slot="header" class="title"
          >${msg('Create Transaction Request')}</span
        >
        <form
          class="column"
          ${onSubmit(f => {
            console.log(f);
            this.transactionRequestInput = {
              amount: parseFloat(f.amount),
              counterparty: f.counterparty,
              transactionRequestType: f['transaction-request-type'],
            };
            this._dialog.show();
          })}
        >
          <sl-select
            name="transaction-request-type"
            required
            id="request-type"
            value="send"
            style="margin-bottom: 16px;"
            @sl-change=${() => this.requestUpdate()}
          >
            <sl-option value="send">${msg('Send')}</sl-option>
            <sl-option value="receive">${msg('Receive')}</sl-option>
          </sl-select>

          <sl-input
            style="margin-bottom: 16px;"
            .label=${msg('Amount')}
            type="number"
            name="amount"
            min="0.1"
            step="0.1"
            value="1.0"
            required
          ></sl-input>

          <search-agent
            style="margin-bottom: 16px;"
            name="counterparty"
            required
            .fieldLabel=${(
              this.shadowRoot?.getElementById('request-type') as any
            )?.value === 'receive'
              ? msg('From')
              : msg('To')}
          ></search-agent>

          <sl-button type="submit" variant="primary"
            >${msg('Create Transaction Request')}</sl-button
          >
        </form>
      </sl-card>
    `;
  }

  static styles = sharedStyles;
}
