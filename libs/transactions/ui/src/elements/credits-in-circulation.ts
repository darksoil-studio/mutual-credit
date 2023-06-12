import { sharedStyles } from '@holochain-open-dev/elements';
import {
  ProfilesStore,
  profilesStoreContext,
} from '@holochain-open-dev/profiles';
import {
  asyncDerived,
  completed,
  pipe,
  sliceAndJoin,
  StoreSubscriber,
} from '@holochain-open-dev/stores';
import { consume } from '@lit-labs/context';
import { msg } from '@lit/localize';
import { html, LitElement } from 'lit';
import { customElement } from 'lit/decorators.js';
import { transactionsStoreContext } from '../context';
import { TransactionsStore } from '../transactions-store';

@customElement('credits-in-circulation')
export class CreditsInCirculation extends LitElement {
  /**
   * @internal
   */
  @consume({ context: transactionsStoreContext })
  transactionsStore!: TransactionsStore;

  /**
   * @internal
   */
  @consume({ context: profilesStoreContext })
  profilesStore!: ProfilesStore;

  /**
   * @internal
   */
  _creditInCirculation = new StoreSubscriber(
    this,
    () =>
      pipe(
        this.profilesStore.agentsWithProfile,
        agents => sliceAndJoin(this.transactionsStore.balanceForAgent, agents),
        allBalances =>
          completed(
            Array.from(allBalances.values()).reduce(
              (acc, next) => (next > 0 ? acc + next : acc),
              0
            )
          )
      ),

    () => []
  );

  renderBalance(credits: number) {
    const roundedCredits = Math.round(credits * 100) / 100;
    return html`
      <div class="column center-content" style="flex: 1;">
        <span style="font-size: 24px; margin: 16px;">
          ${roundedCredits} ${msg('credits')}
        </span>
      </div>
    `;
  }

  render() {
    switch (this._creditInCirculation.value.status) {
      case 'pending':
        return html`
          <sl-skeleton effect="pulse" style="margin: 8px"></sl-skeleton>
        `;
      case 'complete':
        return this.renderBalance(this._creditInCirculation.value.value);
      case 'error':
        return html`
          <display-error
            .headline=${msg('Error fetching the balance')}
            .error=${this._creditInCirculation.value.error}
          ></display-error>
        `;
    }
  }

  static styles = [sharedStyles];
}
