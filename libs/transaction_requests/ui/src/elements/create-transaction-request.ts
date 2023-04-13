import { html, LitElement } from 'lit';
import { customElement, property, query } from 'lit/decorators.js';

import '@shoelace-style/shoelace/dist/components/button/button.js';
import '@shoelace-style/shoelace/dist/components/card/card.js';
import '@shoelace-style/shoelace/dist/components/dialog/dialog.js';
import '@holochain-open-dev/profiles/dist/elements/search-agent.js';

import { Profile } from '@holochain-open-dev/profiles';
import { onSubmit, sharedStyles } from '@holochain-open-dev/elements';
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

  @property({ type: Object })
  _recipientAgent: AgentPubKey | undefined = undefined;

  @property()
  _amount: number | undefined;

  async createTransactionRequest() {
    await this.transactionRequestsStore.client.createTransactionRequest(
      { Send: null },
      this._recipientAgent!,
      this._amount!
    );

    this.dispatchEvent(
      new CustomEvent('transaction-request-created', {
        detail: { recipientPubKey: this._recipientAgent, amount: this._amount },
        composed: true,
        bubbles: true,
      })
    );
  }

  renderConfirmDialog() {
    return html`
      <sl-dialog id="dialog">
        <span slot="headline">${msg('Confirm Transaction Request')}</span>
        <span>
          You are about to create an offer to
          <agent-avatar .agentPubKey=${this._recipientAgent}></agent-avatar>.
          This would lower your balance by ${this._amount} and raise the
          recipient's value by the same amount.
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
            this._amount = f.amount;
            this._recipientAgent = f.recipient;
            this._dialog.show();
          })}
        >
          <span class="title" style="margin-bottom: 8px;"
            >${msg('Create Transaction Request')}</span
          >
          <search-agent
            name="recipient"
            required
            .fieldLabel=${msg('Recipient')}
          ></search-agent>

          <sl-textfield
            style="padding-top: 16px; margin-bottom: 16px;"
            .label=${msg('Amount')}
            type="number"
            name="amount"
            min="0.1"
            step="0.1"
            required
          ></sl-textfield>

          <sl-button type="submit">${msg('Create Offer')}</sl-button>
        </form>
      </sl-card>
    `;
  }

  static styles = sharedStyles;
}
