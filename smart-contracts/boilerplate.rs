#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait BoilerPlate {
    
    fn get_current_time(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    fn add(&self, val1: BigUint, val2: BigUint) -> BigUint {
        val1 + val2
    }

    #[init]
    fn init(&self) {
        let my_address = &self.blockchain().get_caller();
        self.owner().set(my_address);
        let contract = self.blockchain().get_sc_address();
        self.contract().set(contract);
        
    }

    #[endpoint(camelCaseEndpointName)]
    fn snake_case_method_name(&self, value: &BigInt) {}

    fn private_method(&self, value: &BigInt) {}

    #[view(getData)]
    fn get_data(&self) -> u32{
        0
    }
     
    #[endpoint(getContractAddress)]
    fn get_contract_address(&self) {}
     
    #[view(getLockedEgldBalance)]
    fn get_locked_egld_balance(&self) -> BigUint {
        self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0)
    }

    #[storage_mapper("owner")]
    fn owner(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("contract")]
    fn contract(&self) -> SingleValueMapper<ManagedAddress>;

}
