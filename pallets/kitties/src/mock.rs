use crate as pallet_kitties;
use frame_support::traits::{ConstU16, ConstU64, ConstU32};
use sp_core::H256;
use sp_runtime::{
    TokenError,
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use frame_support::{
    traits::ConstU128,
    PalletId
};
use pallet_insecure_randomness_collective_flip;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;/// Balance of an account.
pub type Balance = u128;

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

pub const BALANCE_RAW: u128 = 100 * EXISTENTIAL_DEPOSIT;

frame_support::parameter_types! {
	pub KittyPalletId: PalletId = PalletId(*b"py/kitty");
	pub KittyPrice: Balance = EXISTENTIAL_DEPOSIT * 10;
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		KittiesModule: pallet_kitties,
        Randomness: pallet_insecure_randomness_collective_flip,
		Balances: pallet_balances,
	}
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type HoldIdentifier = ();
    type MaxHolds = ();
}


impl pallet_kitties::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Randomness = Randomness;
    type Currency = Balances;
    type KittyPrice = KittyPrice;
    type PalletId = KittyPalletId;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {

}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances:
            vec![
                (1, BALANCE_RAW),
                (2, BALANCE_RAW),
                (3, BALANCE_RAW),
                (4, BALANCE_RAW),
                (12, 10 * EXISTENTIAL_DEPOSIT),
            ]
    }
        .assimilate_storage(&mut t)
        .unwrap();
    // let mut ext: sp_io::TestExternalities = sp_io::TestExternalities::new(t.into());
    let mut ext: sp_io::TestExternalities = t.into();
    // let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}