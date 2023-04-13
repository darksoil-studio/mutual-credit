import { css, html, property } from 'lit-element';
import { Button } from 'scoped-material-components/mwc-button';
import { CircularProgress } from 'scoped-material-components/mwc-circular-progress';
import { sharedStyles } from './utils/shared-styles';
import { Offer } from '../types';
import { TransactorElement } from './utils/transactor-element';

export abstract class OfferDetail extends TransactorElement {
  /** Public attributes */

  /**
   * The offer to show the details of
   * This argument is mandatory, either in property or in attribute form
   */
  @property({ type: String, attribute: 'offer-hash' })
  offerHash!: string;

  /** Private properties */

  @property({ type: Boolean })
  _loading = true;

  @property({ type: Boolean })
  _accepting = false;

  @property({ type: Boolean })
  _rejecting = false;

  @property({ type: Boolean })
  _canceling = false;

  static styles = [
    sharedStyles,
    css`
      :host {
        display: flex;
      }
    `,
  ];

  /** Actions */

  async firstUpdated() {
    await this._deps.store.fetchMyPendingOffers();
    this._loading = false;
  }

  async acceptOffer() {
    this._accepting = true;

    await this._deps.store.acceptOffer(this.offerHash);

    this.dispatchEvent(
      new CustomEvent('offer-completed', {
        detail: { offerHash: this.offerHash },
        composed: true,
        bubbles: true,
      })
    );
    this._accepting = false;
  }

  get offer(): Offer {
    return this._deps.store.offer(this.offerHash);
  }

  /** Renders */

  render() {
    if (this._loading || this._accepting || this._canceling || this._rejecting)
      return html`<div class="column fill center-content">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
        <span style="margin-top: 18px;" class="placeholder"
          >${this.placeholderMessage()}</span
        >
      </div>`;

    return html`
      <div class="column">
        ${this.renderCounterparty()}
        <div class="row center-content">${this.renderAcceptOffer()}</div>
      </div>
    `;
  }

  renderCounterparty() {
    return html`
      <div class="row" style="flex: 1;">
        <div class="column">
          <span class="item title">
            Offer ${this._deps.store.isOutgoing(this.offer) ? ' to ' : ' from '}
            ${this._deps.store.counterpartyNickname(this.offer)}
          </span>
          <span class="item">Agend ID: ${this.offer.recipient_pub_key}</span>

          <span class="item">
            Transaction amount: ${this.offer.amount} credits
          </span>
        </div>
      </div>
    `;
  }

  placeholderMessage() {
    if (this._accepting) return 'Accepting offer...';
    if (this._canceling) return 'Canceling offer...';
    if (this._rejecting) return 'Rejecting offer...';
    return 'Loading offer...';
  }

  renderAcceptOffer() {
    if (this._deps.store.isOutgoing(this.offer)) {
      return html`<mwc-button
        style="flex: 1;"
        label="Awaiting for approval"
        disabled
        raised
      >
      </mwc-button>`;
    } else {
      return html`
        <mwc-button
          style="flex: 1;"
          label="ACCEPT OFFER"
          raised
          @click=${() => this.acceptOffer()}
        ></mwc-button>
      `;
    }
  }

  getScopedElements() {
    return {
      'mwc-button': Button,
      'mwc-circular-progress': CircularProgress,
    };
  }
}
