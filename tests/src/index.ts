
import { Orchestrator } from "@holochain/tryorama";

import hc_mixin_mutual_credit from './mutual-credit/hc_mixin_mutual_credit';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
hc_mixin_mutual_credit(orchestrator);
orchestrator.run();



