import { css, html, LitElement } from "lit";
import { provide } from "@lit-labs/context";
import { property, customElement } from "lit/decorators.js";

import { transactionsStoreContext } from "../context.js";
import { TransactionsStore } from "../transactions-store.js";

@customElement("transactions-context")
export class TransactionsContext extends LitElement {
  @provide({ context: transactionsStoreContext })
  @property({ type: Object })
  store!: TransactionsStore;

  render() {
    return html`<slot></slot>`;
  }

  static styles = css`
    :host {
      display: contents;
    }
  `;
}
