/**
 * bidx.test.ts — single entry point
 *
 * Platform PDAs (config, authenticators_registry) are global singletons.
 * We initialize ONCE here in the root before() and pass the context down
 * to every suite. This avoids "already in use" errors across test files.
 *
 * Run with:
 *   anchor test
 * or a single suite:
 *   yarn ts-mocha -p ./tsconfig.json -t 1000000 tests/bidx.test.ts --grep "place_bid"
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bidx } from "../target/types/bidx";
import { setupPlatform, PlatformContext } from "./helpers";

// Import suite functions
import { runInitializeTests } from "./initialize";
import { runCreateAuctionTests } from "./create_auction";
import { runPlaceBidTests } from "./place_bid";
import { runSettleAndWithdrawTests } from "./settle_and_withdraw";

describe("bidx", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Bidx as Program<Bidx>;
  const connection = provider.connection;

  // Shared across ALL suites — initialized exactly once
  let platform: PlatformContext;

  before("initialize platform (runs once)", async () => {
    platform = await setupPlatform(program, connection);
  });

  // Each suite receives the shared platform context
  runInitializeTests(() => ({ program, connection, platform }));
  runCreateAuctionTests(() => ({ program, connection, platform }));
  runPlaceBidTests(() => ({ program, connection, platform }));
  runSettleAndWithdrawTests(() => ({ program, connection, platform }));
});
