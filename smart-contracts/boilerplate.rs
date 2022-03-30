#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Boilerplate {
     #[init]
    fn init(&self) {}

    #[endpoint(camelCaseEndpointName)]
    fn snake_case_method_name(&self, value: &BigInt) {
    }

    fn private_method(&self, value: &BigInt) {
    }

    #[view(getData)]
    fn get_data(&self) -> u32{
        0
    }
     
    #[endpoint(getContractAddress)]
    fn get_contract_address(&self){
        let contract = self.blockchain().get_sc_address();
    }
     
    #[view(getLockedEgldBalance)]
    fn get_locked_egld_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::egld(), 0)
    }
     
}
