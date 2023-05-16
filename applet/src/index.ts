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

import {
  ActionHash,
  AppAgentClient,
  CellType,
  EntryHash,
} from '@holochain/client';
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

import './mutual-credit-applet-main.js';
import { ProfilesClient, ProfilesStore } from '@holochain-open-dev/profiles';

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

function appletViews(
  client: AppAgentClient,
  _appletId: EntryHash,
  profilesClient: ProfilesClient,
  weServices: WeServices
): AppletViews {
  return {
    main: element =>
      render(
        wrapAppletView(
          client,
          profilesClient,
          weServices,
          html`<mutual-credit-applet-main></mutual-credit-applet-main>`
        ),
        element
      ),
    blocks: {},
    entries: {
      profiles_integrity: {},
      transaction_requests_integrity: {},
      transactions_integrity: {},
    },
  };
}

function crossAppletViews(
  applets: ReadonlyMap<EntryHash, AppletClients>,
  weServices: WeServices
): CrossAppletViews {
  return {
    main: element => {},
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
