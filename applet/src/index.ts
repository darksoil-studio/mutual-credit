import { TransactionRequestsStore, TransactionRequestsClient } from '@darksoil/mutual-credit-transaction-requests';
import 'lib/mutual_credit/transaction-requests/elements/transaction-requests-context.js';

import { TransactionsStore, TransactionsClient } from 'lib';
import 'lib/mutual_credit/transactions/elements/transactions-context.js';

import { ActionHash, AppAgentClient, CellType } from '@holochain/client';
import { html, render, TemplateResult } from 'lit';
import '@holochain-open-dev/profiles/elements/profiles-context.js';

import {
  CrossGroupViews,
  GroupInfo,
  GroupServices,
  GroupViews,
  GroupWithApplets,
  OpenViews,
  WeApplet,
  WeServices,
} from '@lightningrodlabs/we-applet';
import './main-block';

function wrapGroupView(
  client: AppAgentClient,
  groupInfo: GroupInfo,
  groupServices: GroupServices,
  innerTemplate: TemplateResult
): TemplateResult {
  return html`
    <profiles-context .store=${groupServices.profilesStore}>
      ${innerTemplate}
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
    entries: {}
  profiles_integrity: {},
}
  transaction_requests_integrity: {},
}
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

function wrapTransactionsGroupView(
  client: AppAgentClient,
  groupInfo: GroupInfo,
  groupServices: GroupServices,
  innerTemplate: TemplateResult
): TemplateResult {
  const transactionsStore = new TransactionsStore(new TransactionsClient(client, 'transactions'));
  return wrapGroupView(
    client,
    groupInfo,
    groupServices,
    html`<transactions-context .store=${ transactionsStore}>
      ${innerTemplate}
    </transactions-context>`);
}


function wrapTransactionRequestsGroupView(
  client: AppAgentClient,
  groupInfo: GroupInfo,
  groupServices: GroupServices,
  innerTemplate: TemplateResult
): TemplateResult {
  const transactionRequestsStore = new TransactionRequestsStore(new TransactionRequestsClient(client, 'transaction_requests'));
  return wrapGroupView(
    client,
    groupInfo,
    groupServices,
    html`<transaction-requests-context .store=${ transactionRequestsStore}>
      ${innerTemplate}
    </transaction-requests-context>`);
}


function wrapProfilesGroupView(
  client: AppAgentClient,
  groupInfo: GroupInfo,
  groupServices: GroupServices,
  innerTemplate: TemplateResult
): TemplateResult {
  const profilesStore = new ProfilesStore(new ProfilesClient(client, 'profiles'));
  return wrapGroupView(
    client,
    groupInfo,
    groupServices,
    html`<profiles-context .store=${ profilesStore}>
      ${innerTemplate}
    </profiles-context>`);
}

