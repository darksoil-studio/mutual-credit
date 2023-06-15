import {
  hashProperty,
  notifyError,
  sharedStyles,
} from '@holochain-open-dev/elements';
import { StoreSubscriber } from '@holochain-open-dev/stores';
import '@holochain-open-dev/elements/dist/elements/display-error.js';
import '@holochain-open-dev/profiles/dist/elements/agent-avatar.js';
import '@shoelace-style/shoelace/dist/components/skeleton/skeleton.js';
import { ActionHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { localized, msg } from '@lit/localize';
import { css, html, LitElement } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { transactionRequestsStoreContext } from '../context';
import {
  counterparty,
  isOutgoing,
  TransactionRequestsStore,
} from '../transaction-requests-store';
import { TransactionRequestWithStatus } from '../types';

@localized()
@customElement('transaction-request-detail')
export class TransactionRequestDetail extends LitElement {
  @consume({ context: transactionRequestsStoreContext, subscribe: true })
  transactionRequestsStore!: TransactionRequestsStore;

  /** Public attributes */

  /**
   * REQUIRED: The transaction request to show the details for
   */
  @property(hashProperty('transaction-request-hash'))
  transactionRequestHash!: ActionHash;

  _transactionRequest = new StoreSubscriber(
    this,
    () =>
      this.transactionRequestsStore.transactionRequests.get(
        this.transactionRequestHash
      ),
    () => [this.transactionRequestHash]
  );

  /** Private properties */

  @property({ type: Boolean })
  _accepting = false;

  @property({ type: Boolean })
  _rejecting = false;

  @property({ type: Boolean })
  _cancelling = false;

  /** Actions */

  async acceptTransactionRequest() {
    if (this._rejecting || this._accepting) return;
    this._accepting = true;

    try {
      await this.transactionRequestsStore.client.acceptTransactionRequest(
        this.transactionRequestHash
      );

      this.dispatchEvent(
        new CustomEvent('transaction-completed', {
          detail: { transactionRequestHash: this.transactionRequestHash },
          composed: true,
          bubbles: true,
        })
      );
    } catch (e) {
      notifyError(msg('Error accepting the transaction request'));
      console.error(e);
    }
    this._accepting = false;
  }

  async cancelTransactionRequest() {
    if (this._cancelling) return;

    this._cancelling = true;

    try {
      await this.transactionRequestsStore.client.cancelTransactionRequest(
        this.transactionRequestHash
      );

      this.dispatchEvent(
        new CustomEvent('transaction-request-cancelled', {
          detail: { transactionRequestHash: this.transactionRequestHash },
          composed: true,
          bubbles: true,
        })
      );
    } catch (e) {
      notifyError(msg('Error cancelling the transaction request'));
      console.error(e);
    }
    this._cancelling = false;
  }

  async rejectTransactionRequest() {
    if (this._rejecting || this._accepting) return;
    this._rejecting = true;

    try {
      await this.transactionRequestsStore.client.rejectTransactionRequest(
        this.transactionRequestHash
      );

      this.dispatchEvent(
        new CustomEvent('transaction-request-rejected', {
          detail: { transactionRequestHash: this.transactionRequestHash },
          composed: true,
          bubbles: true,
        })
      );
    } catch (e) {
      notifyError(msg('Error rejecting the transaction request'));
      console.error(e);
    }
    this._rejecting = false;
  }

  /** Renders */

  renderActions(transactionRequest: TransactionRequestWithStatus) {
    if (transactionRequest.status === 'completed')
      return html`<span
        >${msg('This transaction request has already been completed')}</span
      >`;

    if (
      transactionRequest.transactionRequest.action.author.toString() ===
      this.transactionRequestsStore.client.appAgentClient.myPubKey.toString()
    )
      return html`
        <sl-button
          @click=${() => this.cancelTransactionRequest()}
          .loading="${this._cancelling}"
        >
          ${msg('Cancel Request')}
        </sl-button>
        <sl-button variant="primary" disabled style="margin-left: 16px">
          ${msg('Waiting for approval')}
        </sl-button>
      `;
    return html`
      <sl-button
        @click=${() => this.rejectTransactionRequest()}
        .loading="${this._rejecting}"
      >
        ${msg('Reject')}
      </sl-button>
      <sl-button
        style="margin-left: 16px"
        .loading="${this._accepting}"
        variant="primary"
        @click=${() => this.acceptTransactionRequest()}
      >
        ${msg('Accept')}
      </sl-button>
    `;
  }

  renderTransactionRequest(transactionRequest: TransactionRequestWithStatus) {
    return html`
      <div class="column">
        <div class="row" style="align-items: center; margin-bottom: 16px">
          ${isOutgoing(
            this.transactionRequestsStore.client.client.myPubKey,
            transactionRequest.transactionRequest.entry
          )
            ? html`
                <span
                  >${msg('Send')}
                  ${transactionRequest.transactionRequest.entry.amount}
                  ${msg('to')}
                </span>
              `
            : html`
                <span
                  >${msg('Receive')}
                  ${transactionRequest.transactionRequest.entry.amount}
                  ${msg('from')}
                </span>
              `}
          <agent-avatar
            style="margin-left: 8px"
            .agentPubKey=${counterparty(
              this.transactionRequestsStore.client.appAgentClient.myPubKey,
              transactionRequest.transactionRequest.entry
            )}
          ></agent-avatar>
        </div>

        <div class="row">${this.renderActions(transactionRequest)}</div>
      </div>
    `;
  }

  render() {
    switch (this._transactionRequest.value.status) {
      case 'pending':
        return html`
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
        `;
      case 'complete':
        return this.renderTransactionRequest(
          this._transactionRequest.value.value!
        );
      case 'error':
        return html`
          <display-error
            tooltip
            .headline=${msg('Error fetching the transaction history')}
            .error=${this._transactionRequest.value.error.data.data}
          ></display-error>
        `;
    }
  }

  static styles = [
    sharedStyles,
    css`
      :host {
        display: flex;
      }
    `,
  ];
}
