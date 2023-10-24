#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod azeromessage {
    use ink::prelude::string::String;

    #[ink(event)]
    pub struct MessageSent {
        from: AccountId,
        to: AccountId,
        message: String,
        encrypted: bool
    }

    #[ink(storage)]
    pub struct AzeroMessage {
        owner: AccountId,
        fees: u128
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if function can only be called by owner.
        OnlyOwner,
        /// Returned if the transfer amount didn't cover the fees.
        InsufficientTransfer,
    }

    pub type Result<T> = core::result::Result<T, Error>;
    
    impl AzeroMessage {
        /// Constructor that initializes the contract with the caller as a owner and fees at 0.05 AZERO.
        #[ink(constructor)]
        pub fn default() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                fees: 50_000_000_000
            }
        }

        /// Function to emit a message event on the blockchain.
        #[ink(message, payable)]
        pub fn send_message(&mut self, address: AccountId, message: String, encrypted: bool ) -> Result<()> {
            let _transferred = self.env().transferred_value();
            if _transferred < self.fees {
                return Err(Error::InsufficientTransfer);
            }

            Self::env().emit_event(MessageSent {
                from: self.env().caller(),
                to: address,
                message: message,
                encrypted: encrypted
            });
            Ok(())
            
        }
        /// Function to collect the fees accumulated in the contract.
        #[ink(message)]
        pub fn collect_fees(&mut self) {
            let balance = self.env().balance();
            assert!(balance > 10_000_000_000, "Balance too low to withdraw already!");

            let amount_to_transfer = balance - 10_000_000_000;
            self.env().transfer(self.owner,amount_to_transfer).ok();
            
        }

        /// Function to modify the owner of the contract (only usable by current owner)
        #[ink(message)]
        pub fn modify_owner(&mut self, address: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::OnlyOwner);
            }
            self.owner = address;
            Ok(())                
        }
        /// Function to modify the messaging fee of the contract (only usable by current owner)
        #[ink(message)]
        pub fn modify_fees(&mut self, value: u128) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::OnlyOwner);
            } 
            self.fees = value;
            Ok(())        
        }

        /// Function to query the current owner.
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner               
        }
        
        
        /// Function to query the current messaging fee.
        #[ink(message)]
        pub fn get_fees(&self) -> u128 {
            self.fees
        }

    }
}
