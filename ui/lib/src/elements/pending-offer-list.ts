import { html, property, css } from 'lit-element';

import { List } from 'scoped-material-components/mwc-list';
import { ListItem } from 'scoped-material-components/mwc-list-item';
import { CircularProgress } from 'scoped-material-components/mwc-circular-progress';

import { Offer } from '../types';
import { sharedStyles } from './utils/shared-styles';
import { HoloHashed } from '@holochain-open-dev/core-types';
import { Icon } from 'scoped-material-components/mwc-icon';
import { TransactorStore } from '../transactor.store';
import { TransactorElement } from './utils/transactor-element';

export abstract class PendingOfferList extends TransactorElement {
  /** Public attributes */

  /** Private properties */

  @property({ type: String })
  _lastSelectedOfferHash: string | undefined = undefined;

  @property({ type: String })
  _loading = true;

  static styles = [
    sharedStyles,
    css`
      :host {
        display: flex;
      }
    `,
  ];

  async firstUpdated() {
    await this._deps.store.fetchMyPendingOffers();
    this._loading = false;
  }

  renderPlaceholder(type: string) {
    return html`
      <div class="column center-content" style="flex: 1">
        <span style="padding-top: 34px;" class="placeholder padding">
          You have no ${type.toLowerCase()} offers
        </span>
      </div>
    `;
  }

  offerSelected(offerHash: string) {
    this.dispatchEvent(
      new CustomEvent('offer-selected', {
        detail: { offerHash, composed: true, bubbles: true },
      })
    );
    this._lastSelectedOfferHash = offerHash;
  }

  renderOfferList(title: string, offers: Array<HoloHashed<Offer>>) {
    return html`<div class="column">
      <span class="title">${title} offers</span>

      ${offers.length === 0
        ? this.renderPlaceholder(title)
        : html`
            <mwc-list>
              ${offers.map(
                (offer, index) => html`
                  <mwc-list-item
                    @click=${() => this.offerSelected(offer.hash)}
                    graphic="avatar"
                    .activated=${this._lastSelectedOfferHash
                      ? this._lastSelectedOfferHash === offer.hash
                      : false}
                  >
                    <span>
                      ${offer.content.amount} credits
                      ${this._deps.store.isOutgoing(offer.content) ? 'to' : 'from'}
                      ${this._deps.store.counterpartyNickname(offer.content)}
                    </span>

                    <mwc-icon
                      slot="graphic"
                      .style="color: ${this._deps.store.isOutgoing(offer.content)
                        ? 'red'
                        : 'green'}"
                      >${this._deps.store.isOutgoing(offer.content)
                        ? 'call_made'
                        : 'call_received'}</mwc-icon
                    >
                  </mwc-list-item>
                  ${index < offers.length - 1
                    ? html`<li divider padded role="separator"></li> `
                    : html``}
                `
              )}
            </mwc-list>
          `}
    </div>`;
  }

  render() {
    if (this._loading)
      return html`<div class="column fill center-content">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
        <span class="placeholder" style="margin-top: 18px;"
          >Fetching pending offers...</span
        >
      </div>`;

    return html`<div class="column fill">
      <div style="margin-bottom: 24px;">
        ${this.renderOfferList('Incoming', this._deps.store.incomingOffers)}
      </div>
      ${this.renderOfferList('Outgoing', this._deps.store.outgoingOffers)}
    </div>`;
  }

  getScopedElements() {
    return {
      'mwc-circular-progress': CircularProgress,
      'mwc-list': List,
      'mwc-list-item': ListItem,
      'mwc-icon': Icon,
    };
  }
}
