#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + TypeInfo {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type MaxNomeLength: Get<u32>;
        type MaxTelefoneLength: Get<u32>;
        type MaxEmailLength: Get<u32>;
        type MaxDataLength: Get<u32>;
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Contato<T: Config> {
        id: u32,
        nome: BoundedVec<u8, T::MaxNomeLength>,
        telefone: BoundedVec<u8, T::MaxTelefoneLength>,
        email: BoundedVec<u8, T::MaxEmailLength>,
        idade: u32,
        data_aniversario: BoundedVec<u8, T::MaxDataLength>,
        categoria: Categoria,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Categoria {
        Amigo,
        Familiar,
        Colega,
        Outro,
    }

    #[pallet::storage]
    #[pallet::getter(fn contatos)]
    pub type Contatos<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, Contato<T>>;

    #[pallet::storage]
    #[pallet::getter(fn contador_contatos)]
    pub type ContadorContatos<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ContatoCriado { quem: T::AccountId, id: u32 },
        ContatoAtualizado { quem: T::AccountId, id: u32 },
        ContatoDeletado { quem: T::AccountId, id: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        ContatoNaoEncontrado,
        NomeMuitoLongo,
        TelefoneMuitoLongo,
        EmailMuitoLongo,
        DataMuitoLonga,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(Weight::default())]
        #[pallet::call_index(0)]
        pub fn criar_contato(
            origin: OriginFor<T>,
            nome: Vec<u8>,
            telefone: Vec<u8>,
            email: Vec<u8>,
            idade: u32,
            data_aniversario: Vec<u8>,
            categoria: Categoria,
        ) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            let id = ContadorContatos::<T>::get(&quem);

            let contato = Contato {
                id,
                nome: BoundedVec::try_from(nome).map_err(|_| Error::<T>::NomeMuitoLongo)?,
                telefone: BoundedVec::try_from(telefone)
                    .map_err(|_| Error::<T>::TelefoneMuitoLongo)?,
                email: BoundedVec::try_from(email).map_err(|_| Error::<T>::EmailMuitoLongo)?,
                idade,
                data_aniversario: BoundedVec::try_from(data_aniversario)
                    .map_err(|_| Error::<T>::DataMuitoLonga)?,
                categoria,
            };

            Contatos::<T>::insert(&quem, id, contato);
            ContadorContatos::<T>::insert(&quem, id + 1);
            Self::deposit_event(Event::ContatoCriado { quem, id });
            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(1)]
        pub fn atualizar_contato(
            origin: OriginFor<T>,
            id: u32,
            nome: Vec<u8>,
            telefone: Vec<u8>,
            email: Vec<u8>,
            idade: u32,
            data_aniversario: Vec<u8>,
            categoria: Categoria,
        ) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            ensure!(
                Contatos::<T>::contains_key(&quem, id),
                Error::<T>::ContatoNaoEncontrado
            );

            let contato = Contato {
                id,
                nome: BoundedVec::try_from(nome).map_err(|_| Error::<T>::NomeMuitoLongo)?,
                telefone: BoundedVec::try_from(telefone)
                    .map_err(|_| Error::<T>::TelefoneMuitoLongo)?,
                email: BoundedVec::try_from(email).map_err(|_| Error::<T>::EmailMuitoLongo)?,
                idade,
                data_aniversario: BoundedVec::try_from(data_aniversario)
                    .map_err(|_| Error::<T>::DataMuitoLonga)?,
                categoria,
            };

            Contatos::<T>::insert(&quem, id, contato);
            Self::deposit_event(Event::ContatoAtualizado { quem, id });
            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(2)]
        pub fn deletar_contato(origin: OriginFor<T>, id: u32) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            ensure!(
                Contatos::<T>::contains_key(&quem, id),
                Error::<T>::ContatoNaoEncontrado
            );
            Contatos::<T>::remove(&quem, id);
            Self::deposit_event(Event::ContatoDeletado { quem, id });
            Ok(())
        }
    }
}
