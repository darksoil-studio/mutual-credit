import { css, html, LitElement } from "lit";
import { provide } from "@lit-labs/context";
import { property, customElement } from "lit/decorators.js";

import { transactionRequestsStoreContext } from "../context.js";
import { TransactionRequestsStore } from "../transaction-requests-store.js";

@customElement("transaction-requests-context")
export class TransactionRequestsContext extends LitElement {
  @provide({ context: transactionRequestsStoreContext })
  @property({ type: Object })
  store!: TransactionRequestsStore;

  render() {
    return html`<slot></slot>`;
  }

  static styles = css`
    :host {
      display: contents;
    }
  `;
}
