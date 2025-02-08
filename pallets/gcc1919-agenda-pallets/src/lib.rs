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

    // Definindo a estrutura Contato
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Contato<T: Config> {
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

    #[pallet::config]
    pub trait Config: frame_system::Config + TypeInfo {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type MaxNomeLength: Get<u32>;
        type MaxTelefoneLength: Get<u32>;
        type MaxEmailLength: Get<u32>;
        type MaxDataLength: Get<u32>;
    }


    // Storage para armazenar os contatos
    #[pallet::storage]
    #[pallet::getter(fn contatos)]
    pub type Contatos<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Contato<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ContatoCriado { quem: T::AccountId },
        ContatoAtualizado { quem: T::AccountId },
        ContatoDeletado { quem: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        ContatoNaoEncontrado,
        ContatoJaExiste,
        NomeMuitoLongo,
        TelefoneMuitoLongo,
        EmailMuitoLongo,
        DataMuitoLonga,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
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
            ensure!(!Contatos::<T>::contains_key(&quem), Error::<T>::ContatoJaExiste);
            
            let contato = Contato {
                nome: BoundedVec::try_from(nome).map_err(|_| Error::<T>::NomeMuitoLongo)?,
                telefone: BoundedVec::try_from(telefone).map_err(|_| Error::<T>::TelefoneMuitoLongo)?,
                email: BoundedVec::try_from(email).map_err(|_| Error::<T>::EmailMuitoLongo)?,
                idade,
                data_aniversario: BoundedVec::try_from(data_aniversario).map_err(|_| Error::<T>::DataMuitoLonga)?,
                categoria,
            };

            Contatos::<T>::insert(&quem, contato);
            Self::deposit_event(Event::ContatoCriado { quem });

            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(1)]
        pub fn atualizar_contato(
            origin: OriginFor<T>,
            nome: Vec<u8>,
            telefone: Vec<u8>,
            email: Vec<u8>,
            idade: u32,
            data_aniversario: Vec<u8>,
            categoria: Categoria,
        ) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            let mut contato = Contatos::<T>::get(&quem).ok_or(Error::<T>::ContatoNaoEncontrado)?;

            contato.nome = BoundedVec::try_from(nome).map_err(|_| Error::<T>::NomeMuitoLongo)?;
            contato.telefone = BoundedVec::try_from(telefone).map_err(|_| Error::<T>::TelefoneMuitoLongo)?;
            contato.email = BoundedVec::try_from(email).map_err(|_| Error::<T>::EmailMuitoLongo)?;
            contato.idade = idade;
            contato.data_aniversario = BoundedVec::try_from(data_aniversario).map_err(|_| Error::<T>::DataMuitoLonga)?;
            contato.categoria = categoria;

            Contatos::<T>::insert(&quem, contato);
            Self::deposit_event(Event::ContatoAtualizado { quem });

            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(2)]
        pub fn deletar_contato(origin: OriginFor<T>) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            ensure!(Contatos::<T>::contains_key(&quem), Error::<T>::ContatoNaoEncontrado);
            Contatos::<T>::remove(&quem);
            Self::deposit_event(Event::ContatoDeletado { quem });
            Ok(())
        }
    }
}
