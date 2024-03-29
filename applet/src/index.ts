import {
  TransactionRequestsStore,
  TransactionRequestsClient,
} from '@darksoil/mutual-credit-transaction-requests';
import '@darksoil/mutual-credit-transaction-requests/dist/elements/transaction-requests-context.js';

import {
  TransactionsStore,
  TransactionsClient,
} from '@darksoil/mutual-credit-transactions';
import '@darksoil/mutual-credit-transactions/dist/elements/transactions-context.js';
import '@darksoil/mutual-credit-transactions/dist/elements/credits-in-circulation.js';

import { AppAgentClient, EntryHash } from '@holochain/client';
import { html, render, TemplateResult } from 'lit';
import '@holochain-open-dev/profiles/dist/elements/profiles-context.js';

import {
  AppletClients,
  AppletViews,
  CrossAppletViews,
  WeApplet,
  WeServices,
} from '@lightningrodlabs/we-applet';
import '@lightningrodlabs/we-applet/dist/elements/we-services-context.js';
import { ProfilesClient, ProfilesStore } from '@holochain-open-dev/profiles';

import './applet-main.js';
import './cross-applet-main.js';
import { mdiCurrencySign } from '@mdi/js';
import { wrapPathInSvgWithoutPrefix } from '@holochain-open-dev/elements';
import { msg } from '@lit/localize';

function wrapAppletView(
  client: AppAgentClient,
  profilesClient: ProfilesClient,
  weServices: WeServices,
  innerTemplate: TemplateResult
): TemplateResult {
  const transactionsClient = new TransactionsClient(client, 'mutual_credit');
  const transactionsStore = new TransactionsStore(transactionsClient);
  const transactionRequestsStore = new TransactionRequestsStore(
    new TransactionRequestsClient(client, 'mutual_credit'),
    transactionsClient
  );
  return html` <we-services-context .services=${weServices}>
    <profiles-context .store=${new ProfilesStore(profilesClient)}>
      <transactions-context .store=${transactionsStore}>
        <transaction-requests-context .store=${transactionRequestsStore}>
          ${innerTemplate}
        </transaction-requests-context>
      </transactions-context>
    </profiles-context></we-services-context
  >`;
}

async function appletViews(
  client: AppAgentClient,
  _appletId: EntryHash,
  profilesClient: ProfilesClient,
  weServices: WeServices
): Promise<AppletViews> {
  return {
    main: element =>
      render(
        wrapAppletView(
          client,
          profilesClient,
          weServices,
          html`<applet-main></applet-main>`
        ),
        element
      ),
    blocks: {
      credits_in_circulation: {
        label: msg('Credits in circulation'),
        icon_src: wrapPathInSvgWithoutPrefix(mdiCurrencySign),
        view(element, context) {
          render(
            wrapAppletView(
              client,
              profilesClient,
              weServices,
              html`<credits-in-circulation></credits-in-circulation>`
            ),
            element
          );
        },
      },
    },
    entries: {
      transaction_requests_integrity: {},
      transactions_integrity: {},
    },
  };
}

async function crossAppletViews(
  applets: ReadonlyMap<EntryHash, AppletClients>,
  weServices: WeServices
): Promise<CrossAppletViews> {
  return {
    main: element =>
      render(
        html`
          <we-services-context .services=${weServices}>
            <cross-applet-main .applets=${applets}></cross-applet-main
          ></we-services-context>
        `,
        element
      ),
    blocks: {},
  };
}

const applet: WeApplet = {
  appletViews,
  crossAppletViews,
  attachmentTypes: async () => ({}),
  search: async () => [],
};

export default applet;
