import { createContext } from "@lit-labs/context";
import { MutualCreditStore } from "./mutual-credit-store";

export const mutualCreditStoreContext = createContext<MutualCreditStore>("hc_zome_mutual_credit/store_context");
