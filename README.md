## What is rusty-debt?

Rusty-debt is a CLI-based educational project created as an introduction to the Rust language. It allows you to track in- and outgoing debt, so you will never miss out on another heated argument with your friend Joe about the 20 bucks he borrowed but never paid you back.

## How to use rusty-debt

Rusty-debt is simple to use and contains only a few commands:

 * Ability to add debt records which track the creditor, debtor and amount of money owed, as well as a note:<br>
   `rusty-debt add --creditor <creditor> --debtor <debtor> --amount <amount> --description <optional:description>` or<br>
   `rusty-debt add -c <creditor> -d <debtor> -a <amount> --description <optional:description>`
 * Ability to remove debt records when they are paid:<br>
   `rusty-debt remove --index <index>` or<br>
   `rusty-debt remove -i <index>`
 * Ability to reset the debt database and remove all debt:<br>
   `rusty-debt reset`
 * Ability to view all debt records (this is how you obtain the index used with the remove command):<br>
   `rusty-debt print`

## How to build rusty-debt

1. Clone this repository
2. Open a terminal window in the root directory of rusty-debt
3. Run `cargo build`

The output directory (_/debug_ or _/target_ by default) will now have a rusty-debt executable in it).
