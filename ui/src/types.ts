export interface Multiparty {
  spender_pub_key: string;
  recipient_pub_key: string;
}

export interface Transaction extends Multiparty {
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

export interface Offer extends Multiparty {
  amount: number;

  state: OfferState;
}
