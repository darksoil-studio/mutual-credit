import { html, property, query } from 'lit-element';

import { TextField, Card, Button, Dialog } from '@scoped-elements/material-web';
import { AgentProfile, SearchAgent } from '@holochain-open-dev/profiles';

import { sharedStyles } from './utils/shared-styles';

export class CreateOffer extends ScopedElementsMixin(LitElement) {
  /** Private properties */

  @query('#amount')
  _amountField!: TextField;

  @query('#dialog')
  _dialog!: Dialog;

  @property({ type: Object })
  _recipientAgentProfile: AgentProfile | undefined = undefined;

  static styles = sharedStyles;

  firstUpdated() {
    this._amountField.validityTransform = newValue => {
      this.requestUpdate();
      try {
        const amount = parseFloat(newValue);
        if (amount > 0) return { valid: true };
      } catch (e) {}
      this._amountField.setCustomValidity(
        `Offer amount has to be greater than 0`
      );
      return {
        valid: false,
      };
    };
  }

  async createOffer() {
    const recipientPubKey = this._recipientAgentProfile
      ?.agent_pub_key as string;
    const amount = parseFloat(this._amountField.value);

    await this._deps.store.createOffer(recipientPubKey, amount);

    this.dispatchEvent(
      new CustomEvent('offer-created', {
        detail: { recipientPubKey, amount },
        composed: true,
        bubbles: true,
      })
    );
  }

  renderConfirmDialog() {
    return html`
      <mwc-dialog heading="Confirm offer" id="dialog">
        <span>
          You are about to create an offer to
          ${this._recipientAgentProfile?.profile.nickname}, with public key
          ${this._recipientAgentProfile?.agent_pub_key}. This would lower your
          balance by the amount of the transaction and raise the recipient's
          value by the same amount.
        </span>

        <mwc-button slot="secondaryAction" dialogAction="cancel">
          Cancel
        </mwc-button>
        <mwc-button
          .disabled=${!this._amountField || !this._amountField.validity.valid}
          slot="primaryAction"
          @click=${() => this.createOffer()}
          dialogAction="create"
        >
          Confirm
        </mwc-button>
      </mwc-dialog>
    `;
  }

  onAgentSelected(e: CustomEvent) {
    this._recipientAgentProfile = e.detail.agent;
  }

  render() {
    return html`
      ${this.renderConfirmDialog()}
      <mwc-card style="width: auto; flex: 1;">
        <div class="column" style="margin: 16px;">
          <span class="title" style="margin-bottom: 8px;"
            >Create New Offer</span
          >
          <search-agent
            field-label="Recipient"
            @agent-selected=${(e: CustomEvent) => this.onAgentSelected(e)}
          ></search-agent>

          <mwc-textfield
            style="padding-top: 16px; margin-bottom: 16px;"
            label="Amount"
            type="number"
            id="amount"
            min="0.1"
            step="0.1"
            autoValidate
            outlined
          ></mwc-textfield>

          <mwc-button
            label="CREATE OFFER"
            .disabled=${!(
              this._recipientAgentProfile && this._amountField.value
            )}
            @click=${() => this._dialog.show()}
          ></mwc-button>
        </div>
      </mwc-card>
    `;
  }

  get scopedElements() {
    return {
      'mwc-textfield': TextField,
      'mwc-card': Card,
      'mwc-button': Button,
      'mwc-dialog': Dialog,
      'search-agent': SearchAgent,
    };
  }
}
