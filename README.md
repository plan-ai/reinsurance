# Reinsurance Contracts

This repository constitutes the smart contracts for tokenized reinsurance of existing insurances. Do note that it is still in development and many features are planned to be added/changed.

# Setting  up the repo

To set up the repository, follow the following steps:

 1. **Git clone the repository:**  Git clone the repository by running the command ```
```console
git clone https://github.com/codebase-reinsurance/reinsurance-contracts.git
```
2. **Install Rust, Solana, and Anchor on your local:** Take a look at the [Anchor Installation Guide](https://www.anchor-lang.com/docs/installation) for more details.

3. **Building the repository:** Run the command  
 ```console
 anchor build --arch sbf
 ```
 to build the contracts

4. **Generate a Solana keypair:** To generate  a Solana keypair, run the command:
 ```console
solana-keygen new
 ```
5. **Deploying the contracts:**  To deploy the contracts on a local Solana validator, 
  ```console
anchor deploy
 ```
 Voila, now you have successfully deployed the repository on a local Solana validator.

# Running the testsuite

Once, you have generated a Solana keypair and the repository built and setup on your local, you can run the typescript test suite using the command 
  ```console
anchor test
 ```
