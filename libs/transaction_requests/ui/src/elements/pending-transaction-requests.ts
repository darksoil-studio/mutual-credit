import { html, css, LitElement } from 'lit';
import { localized, msg } from '@lit/localize';
import { customElement } from 'lit/decorators.js';
import {
  notifyError,
  sharedStyles,
  wrapPathInSvg,
} from '@holochain-open-dev/elements';
import {
  counterparty,
  isOutgoing,
  TransactionRequestsStore,
} from '../transaction-requests-store';
import { transactionRequestsStoreContext } from '../context';
import '@holochain-open-dev/elements/dist/elements/display-error.js';
import '@shoelace-style/shoelace/dist/components/skeleton/skeleton.js';
import { consume } from '@lit-labs/context';
import { EntryRecord, slice } from '@holochain-open-dev/utils';
import { TransactionRequest, TransactionRequestWithStatus } from '../types';
import {
  asyncDeriveStore,
  AsyncReadable,
  joinAsyncMap,
  StoreSubscriber,
} from '@holochain-open-dev/stores';
import { ActionHash } from '@holochain/client';
import { mdiNotificationClearAll } from '@mdi/js';

@localized()
@customElement('pending-transaction-requests')
export class PendingTransactionRequests extends LitElement {
  /** Public attributes */
  @consume({ context: transactionRequestsStoreContext, subscribe: true })
  transactionRequestsStore!: TransactionRequestsStore;

  /**
   * @internal
   */
  myPendingTransactionRequests = new StoreSubscriber(
    this,
    () =>
      asyncDeriveStore(
        this.transactionRequestsStore.myTransactionRequests,
        transactionsRequestsHashes =>
          joinAsyncMap(
            slice(
              this.transactionRequestsStore.transactionRequests,
              transactionsRequestsHashes
            )
          ) as AsyncReadable<
            ReadonlyMap<ActionHash, TransactionRequestWithStatus>
          >
      ),
    () => []
  );

  /** Private properties */

  renderPlaceholder(type: string) {
    return html`
      <div class="column center-content" style="flex: 1">
        <span style="padding-top: 34px;" class="placeholder padding">
          You have no ${type.toLowerCase()}
        </span>
      </div>
    `;
  }

  transactionRequestSelected(transactionRequestHash: ActionHash) {
    this.dispatchEvent(
      new CustomEvent('transaction-request-selected', {
        detail: { transactionRequestHash },
        composed: true,
        bubbles: true,
      })
    );
  }

  async clearTransactionRequests(
    transactionRequests: Array<EntryRecord<TransactionRequest>>
  ) {
    try {
      await this.transactionRequestsStore.client.clearTransactionRequests(
        transactionRequests.map(r => r.actionHash)
      );
    } catch (e) {
      notifyError(msg('Error rejecting the transaction request'));
    }
  }

  renderTransactionRequestList(
    title: string,
    transactionRequests: Array<EntryRecord<TransactionRequest>>,
    clearVisible: boolean
  ) {
    return html`<div class="column">
      <div class="row" style="align-items:center">
        <span class="title" style="flex: 1">${title}</span>
        ${clearVisible
          ? html`
              <sl-icon-button
                .src=${wrapPathInSvg(mdiNotificationClearAll)}
                @click=${() =>
                  this.clearTransactionRequests(transactionRequests)}
              ></sl-icon-button>
            `
          : html``}
      </div>

      ${transactionRequests.length === 0
        ? this.renderPlaceholder(title)
        : html`
            <div class="column">
              ${transactionRequests.map(
                tr => html`
                  <div
                    class="row"
                    @click=${() =>
                      this.transactionRequestSelected(tr.actionHash)}
                  >
                    ${isOutgoing(
                      this.transactionRequestsStore.client.appAgentClient
                        .myPubKey,
                      tr.entry
                    )
                      ? html`
                          <span
                            >${msg('Send')} ${tr.entry.amount} ${msg('to')}
                          </span>
                        `
                      : html`
                          <span
                            >${msg('Receive')} ${tr.entry.amount} ${msg('from')}
                          </span>
                        `}
                    <agent-avatar
                      .agentPubKey=${counterparty(
                        this.transactionRequestsStore.client.appAgentClient
                          .myPubKey,
                        tr.entry
                      )}
                    ></agent-avatar>
                  </div>
                `
              )}
            </div>
          `}
    </div>`;
  }

  render() {
    switch (this.myPendingTransactionRequests.value.status) {
      case 'pending':
        return html`
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
        `;
      case 'complete':
        const transactionRequests =
          this.myPendingTransactionRequests.value.value;
        const incomingRequests = Array.from(transactionRequests.values())
          .filter(
            tr =>
              tr!.status === 'pending' &&
              tr?.transactionRequest.action.author.toString() !==
                this.transactionRequestsStore.client.appAgentClient.myPubKey.toString()
          )
          .map(tr => tr?.transactionRequest);
        const pendingRequests = Array.from(transactionRequests.values())
          .filter(
            tr =>
              tr!.status === 'pending' &&
              tr?.transactionRequest.action.author.toString() ===
                this.transactionRequestsStore.client.appAgentClient.myPubKey.toString()
          )
          .map(tr => tr?.transactionRequest);

        const rejectedRequests = Array.from(transactionRequests.values())
          .filter(
            tr =>
              tr!.status === 'rejected' &&
              tr?.transactionRequest.action.author.toString() ===
                this.transactionRequestsStore.client.appAgentClient.myPubKey.toString()
          )
          .map(tr => tr?.transactionRequest);
        const cancelledRequests = Array.from(transactionRequests.values())
          .filter(
            tr =>
              tr!.status === 'cancelled' &&
              tr?.transactionRequest.action.author.toString() !==
                this.transactionRequestsStore.client.appAgentClient.myPubKey.toString()
          )
          .map(tr => tr?.transactionRequest);

        const completedRequests = Array.from(transactionRequests.values())
          .filter(
            tr =>
              tr!.status === 'completed' &&
              tr?.transactionRequest.action.author.toString() ===
                this.transactionRequestsStore.client.appAgentClient.myPubKey.toString()
          )
          .map(tr => tr?.transactionRequest);

        return html`<div class="row">
          ${this.renderTransactionRequestList(
            msg('Incoming'),
            incomingRequests,
            false
          )}
          ${this.renderTransactionRequestList(
            msg('Pending'),
            pendingRequests,
            false
          )}
          ${this.renderTransactionRequestList(
            msg('Cancelled'),
            cancelledRequests,
            true
          )}
          ${this.renderTransactionRequestList(
            msg('Rejected'),
            rejectedRequests,
            true
          )}
          ${this.renderTransactionRequestList(
            msg('Completed'),
            completedRequests,
            true
          )}
        </div>`;
      case 'error':
        return html`
          <display-error
            tooltip
            .headline=${msg('Error fetching the pending transaction requests')}
            .error=${this.myPendingTransactionRequests.value.error.data.data}
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
