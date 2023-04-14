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

import { ActionHash, AppAgentClient, CellType } from '@holochain/client';
import { html, render, TemplateResult } from 'lit';
import '@holochain-open-dev/profiles/dist/elements/profiles-context.js';

import {
  CrossGroupViews,
  GroupInfo,
  GroupServices,
  GroupViews,
  GroupWithApplets,
  OpenViews,
  WeApplet,
  WeServices,
} from './we-applet';
import './main-block';
import { ProfilesClient, ProfilesStore } from '@holochain-open-dev/profiles';

function wrapGroupView(
  client: AppAgentClient,
  groupInfo: GroupInfo,
  groupServices: GroupServices,
  innerTemplate: TemplateResult
): TemplateResult {
  const transactionsClient = new TransactionsClient(client, 'transactions');
  const transactionsStore = new TransactionsStore(transactionsClient);
  const transactionRequestsStore = new TransactionRequestsStore(
    new TransactionRequestsClient(client, 'transaction_requests'),
    transactionsClient
  );
  return html` <profiles-context .store=${groupServices.profilesStore}>
    <transactions-context .store=${transactionsStore}>
      <transaction-requests-context .store=${transactionRequestsStore}>
        ${innerTemplate}
      </transaction-requests-context>
    </transactions-context>
  </profiles-context>`;
}

function groupViews(
  client: AppAgentClient,
  groupInfo: GroupInfo,
  groupServices: GroupServices,
  weServices: WeServices
): GroupViews {
  return {
    blocks: {
      main: element =>
        render(
          wrapGroupView(
            client,
            groupInfo,
            groupServices,
            html`<main-block></main-block>`
          ),
          element
        ),
    },
    entries: {
      profiles_integrity: {},
      transaction_requests_integrity: {},
      transactions_integrity: {},
    },
  };
}

function crossGroupViews(
  groupWithApplets: GroupWithApplets[]
): CrossGroupViews {
  return {
    blocks: {
      main: element => {},
    },
  };
}

const applet: WeApplet = {
  attachableTypes: [],
  search: async () => [],
  groupViews,
  crossGroupViews,
};

export default applet;
