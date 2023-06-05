import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

import { localized, msg } from '@lit/localize';
import { sharedStyles } from '@holochain-open-dev/elements';
import '@darksoil/mutual-credit-transactions/dist/elements/my-balance.js';

import { AppAgentClient, DnaHash, EntryHash } from '@holochain/client';
import { ProfilesClient, ProfilesStore } from '@holochain-open-dev/profiles';
import {
  AppletInfo,
  getAppletsInfosAndGroupsProfiles,
  GroupProfile,
  WeServices,
  weServicesContext,
} from '@lightningrodlabs/we-applet';
import { lazyLoad, StoreSubscriber } from '@holochain-open-dev/stores';
import { consume } from '@lit-labs/context';
import {
  TransactionsClient,
  TransactionsStore,
} from '@darksoil/mutual-credit-transactions';
import {
  TransactionRequestsClient,
  TransactionRequestsStore,
} from '@darksoil/mutual-credit-transaction-requests';

@localized()
@customElement('cross-applet-main')
export class CrossAppletMain extends LitElement {
  @property()
  applets!: ReadonlyMap<
    EntryHash,
    { appletClient: AppAgentClient; profilesClient: ProfilesClient }
  >;

  @consume({ context: weServicesContext, subscribe: true })
  weServices!: WeServices;

  appletsInfo = new StoreSubscriber(
    this,
    () =>
      lazyLoad(async () =>
        getAppletsInfosAndGroupsProfiles(
          this.weServices,
          Array.from(this.applets.keys())
        )
      ),
    () => []
  );

  renderNotes(
    applets: ReadonlyMap<EntryHash, AppletInfo>,
    groupsProfiles: ReadonlyMap<DnaHash, GroupProfile>
  ) {
    return html`
      <div class="flex-scrollable-parent" style="margin: 16px">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y column" style="align-items: center">
            <div class="column" style="margin: 16px; max-width: 600px">
              ${Array.from(this.applets.entries()).map(
                ([appletId, { appletClient, profilesClient }]) => {
                  const transactionsClient = new TransactionsClient(
                    appletClient,
                    'mutual_credit'
                  );
                  return html`
                    <profiles-context
                      .store=${new ProfilesStore(profilesClient)}
                    >
                      <transactions-context
                        .store=${new TransactionsStore(transactionsClient)}
                      >
                        <transaction-requests-context
                          .store=${new TransactionRequestsStore(
                            new TransactionRequestsClient(
                              appletClient,
                              'mutual_credit'
                            ),
                            transactionsClient
                          )}
                        >
                          <div class="row title" style="align-items: center">
                            <span>${msg('Balance in')} ${msg('in')} </span>
                            ${applets
                              .get(appletId)
                              ?.groupsIds.map(
                                groupId => html`
                                  <img
                                    .src=${groupsProfiles.get(groupId)
                                      ?.logo_src}
                                    alt="group-${groupsProfiles.get(groupId)
                                      ?.name}"
                                    style="margin-right: 4px; height: 32px; width: 32px"
                                  />
                                `
                              )}
                            <span>${applets.get(appletId)?.appletName}</span>
                          </div>
                          <my-balance></my-balance>
                        </transaction-requests-context>
                      </transactions-context>
                    </profiles-context>
                  `;
                }
              )}
            </div>
          </div>
        </div>
      </div>
    `;
  }

  render() {
    switch (this.appletsInfo.value.status) {
      case 'pending':
        return html`<div class="row center-content" style="flex:1">
          <sl-spinner style="font-size: 2rem"></sl-spinner>
        </div>`;
      case 'complete':
        return this.renderNotes(
          this.appletsInfo.value.value.appletsInfos,
          this.appletsInfo.value.value.groupsProfiles
        );
      case 'error':
        return html`<display-error
          .headline=${msg('Error fetching the applets')}
          .error=${this.appletsInfo.value.error}
        ></display-error>`;
    }
  }

  static styles = [
    css`
      :host {
        display: flex;
        flex: 1;
      }
    `,
    sharedStyles,
  ];
}
