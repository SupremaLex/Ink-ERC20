#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    #[ink(storage)]
    pub struct Erc20 {
        owner: AccountId,
        total_suply: Balance,
        balances: ink_storage::collections::HashMap<AcountId, Balance>,
        allowances: ink_storage::collections::HashMap<(AccountId, AccountId), Balance>,
    }
	#[ink(event)]
	pub struct Transfer {}

	#[ink(event)]
	pub struct Approval {}

    impl Erc20 {

        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(Self::env().caller(), initial_supply);
            Self { 
                owner: Self::env().caller();
                total_suply: initial_supply,
                balances,
                allowances: ink_storage::collections::HashMap::new(),
             }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_suply
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn balance_of(&self, token_owner : AccoundId) -> Balance {
            self.balance.get(token_owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowance(&self, token_owner : AccoundId, spender : AccountId) -> Balance {
            self.allowances.get((token_owner, spender)).unwrap_or(&0)
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;
    }
}
