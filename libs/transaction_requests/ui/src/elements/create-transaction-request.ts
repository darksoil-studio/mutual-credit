import { html, LitElement } from 'lit';
import { customElement, property, query } from 'lit/decorators.js';

import '@shoelace-style/shoelace/dist/components/button/button.js';
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

  transactionRequestInput:
    | {
        counterparty: AgentPubKey;
        amount: number;
        transactionRequestType: 'send' | 'receive';
      }
    | undefined;

  async createTransactionRequest() {
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
    } catch (e) {
      notifyError(msg('Error creating the transaction request'));
      console.error(e);
    }
  }

  renderConfirmDialog() {
    return html`
      <sl-dialog id="dialog">
        <span slot="headline">${msg('Confirm Transaction Request')}</span>
        <span>
          You are about to create an transaction request to
          <agent-avatar
            .agentPubKey=${this.transactionRequestInput?.counterparty}
          ></agent-avatar
          >. This would
          ${this.transactionRequestInput?.transactionRequestType === 'send'
            ? 'lower'
            : 'raise'}
          your balance by ${this.transactionRequestInput?.amount} and
          ${this.transactionRequestInput?.transactionRequestType === 'send'
            ? 'raise'
            : 'lower'}
          the agent receiving the request value by the same amount.
        </span>

        <sl-button> ${msg('Cancel')} </sl-button>
        <sl-button @click=${() => this.createTransactionRequest()}>
          ${msg('Confirm')}
        </sl-button>
      </sl-dialog>
    `;
  }

  render() {
    return html`
      ${this.renderConfirmDialog()}
      <sl-card>
        <form
          class="column"
          ${onSubmit(f => {
            this.transactionRequestInput = {
              amount: f.amount,
              counterparty: f.counterparty,
              transactionRequestType: f['transaction-request-type'],
            };
            this._dialog.show();
          })}
        >
          <span class="title" style="margin-bottom: 8px;"
            >${msg('Create Transaction Request')}</span
          >

          <sl-select name="transaction-request-type" required value="send">
            <sl-option value="send">${msg('Send')}</sl-option>
            <sl-option value="receive">${msg('Receive')}</sl-option>
          </sl-select>

          <sl-textfield
            style="padding-top: 16px; margin-bottom: 16px;"
            .label=${msg('Amount')}
            type="number"
            name="amount"
            min="0.1"
            step="0.1"
            required
          ></sl-textfield>

          <search-agent
            name="counterparty"
            required
            .fieldLabel=${msg('Recipient')}
          ></search-agent>

          <sl-button type="submit">${msg('Create Offer')}</sl-button>
        </form>
      </sl-card>
    `;
  }

  static styles = sharedStyles;
}
