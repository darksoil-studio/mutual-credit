import { AgentPubKey } from '@holochain/client';
import { Transaction } from './types';

export function counterparty(
  myPubKey: AgentPubKey,
  transaction: Transaction
): AgentPubKey {
  if (transaction.recipient.agent_pub_key.toString() === myPubKey.toString())
    return transaction.spender.agent_pub_key;
  return transaction.recipient.agent_pub_key;
}

export function isOutgoing(
  myPubKey: AgentPubKey,
  transaction: Transaction
): boolean {
  return transaction.spender.agent_pub_key.toString() === myPubKey.toString();
}
