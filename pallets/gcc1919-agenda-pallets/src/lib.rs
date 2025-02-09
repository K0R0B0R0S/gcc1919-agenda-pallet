#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;
    use scale_info::prelude::string::String;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + TypeInfo {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type MaxNomeLength: Get<u32>;
        type MaxTelefoneLength: Get<u32>;
        type MaxEmailLength: Get<u32>;
        type MaxTituloLength: Get<u32>;
        type MaxHoraLength: Get<u32>;
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Contato<T: Config> {
        pub id: u32,
        pub nome: BoundedVec<u8, T::MaxNomeLength>,
        pub telefone: BoundedVec<u8, T::MaxTelefoneLength>,
        pub email: BoundedVec<u8, T::MaxEmailLength>,
        pub idade: u32,
        pub data_aniversario: u64,
        pub categoria: Categoria,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Categoria {
        Amigo,
        Familiar,
        Colega,
        Outro,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Compromisso<T: Config> {
        pub id: u32,
        pub titulo: BoundedVec<u8, T::MaxTituloLength>,
        pub data: u64,
        pub hora: BoundedVec<u8, T::MaxHoraLength>,
        pub prioridade: Prioridade,
        pub duracao: u32,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Prioridade {
        Alta,
        Media,
        Baixa,
    }

    #[pallet::storage]
    #[pallet::getter(fn contatos)]
    pub type Contatos<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, Contato<T>>;

    #[pallet::storage]
    #[pallet::getter(fn contador_contatos)]
    pub type ContadorContatos<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn compromissos)]
    pub type Compromissos<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, Compromisso<T>>;

    #[pallet::storage]
    #[pallet::getter(fn contador_compromissos)]
    pub type ContadorCompromissos<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ContatoCriado { quem: T::AccountId, id: u32 },
        ContatoAtualizado { quem: T::AccountId, id: u32 },
        ContatoDeletado { quem: T::AccountId, id: u32 },
        CompromissoCriado { quem: T::AccountId, id: u32 },
        CompromissoAtualizado { quem: T::AccountId, id: u32 },
        CompromissoDeletado { quem: T::AccountId, id: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        ContatoNaoEncontrado,
        NomeMuitoLongo,
        TelefoneMuitoLongo,
        EmailMuitoLongo,
        DataInvalida,
        HoraInvalida,
        DuracaoInvalida,
        CompromissoNaoEncontrado,
        TituloMuitoLongo,
    }
    
    impl<T: Config> Pallet<T> {
        pub fn convert_to_timestamp(date_str: Vec<u8>) -> Result<u64, Error<T>> {
            let date_str = core::str::from_utf8(&date_str).map_err(|_| Error::<T>::DataInvalida)?;
            let parts: Vec<&str> = date_str.split('/').collect();
            if parts.len() != 3 {
                return Err(Error::<T>::DataInvalida);
            }
            let day: u32 = parts[0].parse().map_err(|_| Error::<T>::DataInvalida)?;
            let month: u32 = parts[1].parse().map_err(|_| Error::<T>::DataInvalida)?;
            let year: i32 = parts[2].parse().map_err(|_| Error::<T>::DataInvalida)?;
            if month < 1 || month > 12 || day < 1 || day > 31 {
                return Err(Error::<T>::DataInvalida);
            }
            let timestamp = Self::date_to_unix_timestamp(year, month, day)?;
            Ok(timestamp)
        }

        fn date_to_unix_timestamp(year: i32, month: u32, day: u32) -> Result<u64, Error<T>> {
            let mut days = 0;
            for y in 1970..year {
                days += if Self::is_leap_year(y) { 366 } else { 365 };
            }
            let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            for m in 0..(month as usize - 1) {
                days += days_in_month[m];
                if m == 1 && Self::is_leap_year(year) {
                    days += 1;
                }
            }
            days += day - 1;
            let timestamp = days as u64 * 86400;
            Ok(timestamp)
        }

        fn is_leap_year(year: i32) -> bool {
            (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
        }

        pub fn validate_hour_input(hour: Vec<u8>) -> Result<(), Error<T>> {
        
            let hour_str = core::str::from_utf8(&hour).map_err(|_| Error::<T>::HoraInvalida)?;
            let parts: Vec<&str> = hour_str.split(':').collect();
            if parts.len() != 2 {
                return Err(Error::<T>::HoraInvalida);
            }

            let hours: u32 = parts[0].parse().map_err(|_| Error::<T>::HoraInvalida)?;
            let minutes: u32 = parts[1].parse().map_err(|_| Error::<T>::HoraInvalida)?;

            if hours > 23 || minutes > 59 {
                return Err(Error::<T>::HoraInvalida);
            }

            Ok(())
        }

        pub fn validate_duration_input(duration: u64) -> Result<(), Error<T>> {
            // A duração é em minutos, então o valor máximo é 600 minutos (10 horas) e o mínimo é 1 minuto
            if duration < 1 || duration > 600 {
                return Err(Error::<T>::DuracaoInvalida);
            }
            Ok(())
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(Weight::default())]
        #[pallet::call_index(0)]
        pub fn criar_contato(
            origin: OriginFor<T>,
            nome: String,
            telefone: String,
            email: String,
            idade: u32,
            data_aniversario: String,
            categoria: Categoria,
        ) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            let id: u32 = ContadorContatos::<T>::get(&quem);
            let data_aniversario_parsed = Self::convert_to_timestamp(data_aniversario.into_bytes())?;

            let contato = Contato {
                id,
                nome: BoundedVec::try_from(nome.into_bytes()).map_err(|_| Error::<T>::NomeMuitoLongo)?,
                telefone: BoundedVec::try_from(telefone.into_bytes())
                    .map_err(|_| Error::<T>::TelefoneMuitoLongo)?,
                email: BoundedVec::try_from(email.into_bytes()).map_err(|_| Error::<T>::EmailMuitoLongo)?,
                idade,
                data_aniversario: data_aniversario_parsed,
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
            nome: String,
            telefone: String,
            email: String,
            idade: u32,
            data_aniversario: String,
            categoria: Categoria,
        ) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            let data_aniversario_parsed = Self::convert_to_timestamp(data_aniversario.into_bytes())?;
            ensure!(
                Contatos::<T>::contains_key(&quem, id),
                Error::<T>::ContatoNaoEncontrado
            );

            let contato = Contato {
                id,
                nome: BoundedVec::try_from(nome.into_bytes()).map_err(|_| Error::<T>::NomeMuitoLongo)?,
                telefone: BoundedVec::try_from(telefone.into_bytes()).map_err(|_| Error::<T>::TelefoneMuitoLongo)?,
                email: BoundedVec::try_from(email.into_bytes()).map_err(|_| Error::<T>::EmailMuitoLongo)?,
                idade,
                data_aniversario: data_aniversario_parsed,
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

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(3)]
        pub fn criar_compromisso(
            origin: OriginFor<T>,
            titulo: String,
            data: String,
            hora: String,
            prioridade: Prioridade,
            duracao: u32,
        ) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            let id = ContadorCompromissos::<T>::get(&quem);
            let data_parsed = Self::convert_to_timestamp(data.into_bytes())?;
            ensure!(
                Self::validate_hour_input(hora.clone().into_bytes()).is_ok(),
                Error::<T>::HoraInvalida
            );
            ensure!(
                Self::validate_duration_input(duracao.into()).is_ok(),
                Error::<T>::DuracaoInvalida
            );

            let compromisso = Compromisso {
                id,
                titulo: BoundedVec::try_from(titulo.into_bytes()).map_err(|_| Error::<T>::TituloMuitoLongo)?,
                data: data_parsed,
                hora: BoundedVec::try_from(hora.into_bytes()).map_err(|_| Error::<T>::HoraInvalida)?,
                prioridade,
                duracao,
            };

            Compromissos::<T>::insert(&quem, id, compromisso);
            ContadorCompromissos::<T>::insert(&quem, id + 1);
            Self::deposit_event(Event::CompromissoCriado { quem, id });
            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(4)]
        pub fn atualizar_compromisso(
            origin: OriginFor<T>,
            id: u32,
            titulo: String,
            data: String,
            hora: String,
            prioridade: Prioridade,
            duracao: u32,
        ) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            let data_parsed = Self::convert_to_timestamp(data.into_bytes())?;
            ensure!(
                Compromissos::<T>::contains_key(&quem, id),
                Error::<T>::CompromissoNaoEncontrado
            );
            ensure!(
                Self::validate_hour_input(hora.clone().into_bytes()).is_ok(),
                Error::<T>::HoraInvalida
            );
            ensure!(
                Self::validate_duration_input(duracao.into()).is_ok(),
                Error::<T>::DuracaoInvalida
            );

            let compromisso = Compromisso {
                id,
                titulo: BoundedVec::try_from(titulo.into_bytes()).map_err(|_| Error::<T>::TituloMuitoLongo)?,
                data: data_parsed,
                hora: BoundedVec::try_from(hora.into_bytes()).map_err(|_| Error::<T>::HoraInvalida)?,
                prioridade,
                duracao,
            };

            Compromissos::<T>::insert(&quem, id, compromisso);
            Self::deposit_event(Event::CompromissoAtualizado { quem, id });
            Ok(())
        }

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(5)]
        pub fn deletar_compromisso(origin: OriginFor<T>, id: u32) -> DispatchResult {
            let quem = ensure_signed(origin)?;
            ensure!(
                Compromissos::<T>::contains_key(&quem, id),
                Error::<T>::CompromissoNaoEncontrado
            );

            Compromissos::<T>::remove(&quem, id);
            Self::deposit_event(Event::CompromissoDeletado { quem, id });
            Ok(())
        }
    }
}