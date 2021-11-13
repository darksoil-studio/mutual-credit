
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../utils';

export default (orchestrator: Orchestrator<any>) => 
  orchestrator.registerScenario("hc_mixin_mutual_credit tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([config, config]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find(cell => cell.cellRole.includes('/mutual-credit.dna')) as Cell;
    const bob = bob_happ.cells.find(cell => cell.cellRole.includes('/mutual-credit.dna')) as Cell;

    let offers = await alice.call(
      "mutual-credit",
      "query_my_pending_offers",
      null
    );
    t.equal(offers.length, 0);

    await sleep(10);
});
