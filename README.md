# Time-Lock

What Are Time-Bound Tokens and Claimable Balances on Stellar?
Time-bound tokens are digital assets that can only be claimed or accessed after a specific period has passed. This feature is useful for vesting schedules, delayed rewards, or conditional payments in blockchain projects.

Claimable balances are a mechanism on Stellar that allows you to create a balance (e.g., tokens or XLM) that a recipient can claim, but only under certain conditions—such as after a set time has elapsed.

How Does It Work on Stellar?
Create a Claimable Balance:
The sender creates a claimable balance, specifying the recipient and the conditions (e.g., “can only be claimed after July 1, 2025”).

Set Time Bound:
You define a time window (start and/or end time) during which the recipient can claim the balance.

Claim the Balance:
Once the time condition is met, the recipient can claim the tokens using a simple transaction.

Example Use Cases
Token Vesting: Team or advisor tokens that unlock gradually over time.

Airdrops: Tokens distributed to users, but only claimable after a launch date.

Escrow Payments: Payments released only after a project milestone or deadline.

How to Implement in Soroban (Stellar’s Smart Contract Platform)?
You can implement time-bound claimable balances in Soroban smart contracts by:

Storing claimable balances in a contract state, with each balance having an amount, recipient, unlock time, and claim status.

Providing functions to create a claimable balance (with a recipient and unlock time), to claim available balances (if unlock time has passed), and to view claimable amounts.

Benefits for Projects and Users
Security: Funds are locked on-chain, reducing risk of premature access.

Transparency: All conditions and timings are visible and verifiable on the blockchain.

Automation: No need for manual distribution or off-chain agreements.

In summary:
Time-bound tokens and claimable balances on Stellar are powerful tools for any project that needs scheduled, conditional, or trustless token distribution. They are particularly useful for vesting, airdrops, and milestone-based payments, and can be implemented efficiently using Soroban smart contracts.
