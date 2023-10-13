#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod azeromessage {
    use ink::prelude::string::String;

    #[ink(event)]
    pub struct MessageSent {
        from: AccountId,
        to: AccountId,
        message: String,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct AzeroMessage {
        owner: AccountId,
    }


    impl AzeroMessage {
        /// Constructor that initializes the string and address to empty and default.
        #[ink(constructor)]
        pub fn default() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
            }
        }

        
        #[ink(message, payable)]
        pub fn send_message(&mut self, address: AccountId, message: String ) {
            let _transferred = self.env().transferred_value();
            assert!(_transferred > 200_000_000_000, "Fee to send a message is 0.20 AZERO.");
            Self::env().emit_event(MessageSent {
                from: self.env().caller(),
                to: address,
                message: message,
            })
            
        }

        #[ink(message)]
        pub fn collect_fees(&mut self) {
            let balance = self.env().balance();
            assert!(balance > 10_000_000_000, "Balance too low to withdraw already!");

            let amount_to_transfer = balance - 10_000_000_000;
            self.env().transfer(self.owner,amount_to_transfer).ok();
            
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        type Event = <AzeroMessage as ::ink::reflect::ContractEventBase>::Type;

        fn assert_message_sent_event(
            event: &ink::env::test::EmittedEvent,
            expected_from: AccountId,
            expected_to: AccountId,
            expected_message: String,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            let Event::MessageSent(MessageSent { from, to, message }) = decoded_event;
                assert_eq!(from, expected_from, "encountered invalid message.from");
                assert_eq!(to, expected_to, "encountered invalid message.to");
                assert_eq!(message, expected_message, "encountered invalid message text");
        }

        /// We test a simple use case of our contract.
        #[ink_e2e::test]
        fn sending_works() {
            let mut contract = AzeroMessage::default();
            let mut builder = ink_env::test::TestBuilder::default();
            let transferred_amount = 250_000_000_000;
            let accounts: DefaultAccounts<ink_env::DefaultEnvironment> = DefaultAccounts::new(transferred_amount);
            builder.push_execution_context(
                accounts.bob,
                accounts.bob,
                transferred_amount,
                100_000_000, // Gas limit
                ink_env::test::CallData::new(ink_env::test::CallKind::Plain),
            );
            let message_to_send = "hello".to_string();
            assert_eq!(contract.send_message(accounts.bob, message_to_send.clone()), ());
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_message_sent_event(&emitted_events[0], accounts.alice, accounts.bob, message_to_send.clone())
        }
    }
}
