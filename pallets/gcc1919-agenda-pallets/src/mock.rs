pub (crate) use crate as GCC1919AgendaPallet;
use frame_support::{derive_impl, parameter_types};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type CustomPallet = GCC1919AgendaPallet::Pallet<Test>;
}

// System pallet configuration
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;

}

// Definindo os parâmetros para os valores máximos de comprimento de cada campo
parameter_types! {
    pub const MaxNomeLength: u32 = 100;
    pub const MaxTelefoneLength: u32 = 15;
    pub const MaxEmailLength: u32 = 100;
    pub const MaxTituloLength: u32 = 100;
    pub const MaxHoraLength: u32 = 5;
}

impl GCC1919AgendaPallet::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type MaxNomeLength = MaxNomeLength;
    type MaxTelefoneLength = MaxTelefoneLength;
    type MaxEmailLength = MaxEmailLength;
    type MaxTituloLength = MaxTituloLength;
    type MaxHoraLength = MaxHoraLength;
}

// Test externalities initialization
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
