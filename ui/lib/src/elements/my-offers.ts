import { html } from 'lit-html';
import { Card } from 'scoped-material-components/mwc-card';
import { PendingOfferList } from './pending-offer-list';
import { css, property } from 'lit-element';
import { OfferDetail } from './offer-detail';
import { sharedStyles } from './utils/shared-styles';
import { connectDeps } from '@holochain-open-dev/common';
import { TransactorElement } from './utils/transactor-element';

export abstract class MyOffers extends TransactorElement {
  @property({ type: String, attribute: false })
  _offerHash: string | undefined = undefined;

  render() {
    return html`
      <mwc-card style="width: auto; flex: 1;">
        <div class="row" style="flex: 1;">
          <pending-offer-list
            style="flex: 1; margin: 16px;"
            @offer-selected=${(e: CustomEvent) =>
              (this._offerHash = e.detail.offerHash)}
          ></pending-offer-list>
          <span class="vertical-divider"></span>
          ${this._offerHash
            ? html`
                <offer-detail
                  style="flex: 1; margin: 16px;"
                  @offer-completed=${(e: CustomEvent) =>
                    (this._offerHash = undefined)}
                  .offerHash=${this._offerHash}
                ></offer-detail>
              `
            : html`<div class="fill center-content">
                <span class="placeholder" style="margin: 16px;"
                  >Select an offer to see its details</span
                >
              </div>`}
        </div>
      </mwc-card>
    `;
  }

  static get styles() {
    return [
      sharedStyles,
      css`
        .vertical-divider {
          width: 1px;
          opacity: 0.5;
          background-color: black;
          opacity: 0.1;
          margin: 24px 0;
        }
      `,
    ];
  }

  getScopedElements() {
    return {
      'mwc-card': Card,
      'pending-offer-list': connectDeps(PendingOfferList, this._deps),
      'offer-detail': connectDeps(OfferDetail, this._deps),
    };
  }
}
