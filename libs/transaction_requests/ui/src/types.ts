export interface Transaction {
  spender_pub_key: string;
  recipient_pub_key: string;

  amount: number;
  timestamp: number;
  offer_hash: string;
}

export type OfferState =
  | 'Pending'
  | 'Canceled'
  | 'Rejected'
  | 'Completed'
  | 'Approved';

export interface Offer  {
  spender_pub_key: string;
  recipient_pub_key: string;

  amount: number;

  state: OfferState;
}
