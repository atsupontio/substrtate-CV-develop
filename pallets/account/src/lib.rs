#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{pallet_prelude::*, transactional, Twox64Concat, inherent::Vec};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use pallet_utils::{Role, Status};
	
	#[cfg(feature = "std")]
  	use frame_support::serde::{Deserialize, Serialize};

	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Account<T: Config> {
		pub id: AccountOf<T>,
		pub name: Vec<u8>, // developing
		pub role: Role,
		pub status: Status,
		pub org_type: Vec<u8>, // developing
		pub metadata: Vec<u8>, // developing
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn account_storage)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type AccountStorage<T> = StorageMap<_, Twox64Concat, AccountOf<T>, Account<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. []
		AccountRegisted,
		AccountUpdated,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		AlreadyExist,
		/// Errors should have helpful documentation associated with them.
		AccountNotRegistered,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000)]
		pub fn register(origin: OriginFor<T>, name: Vec<u8>, role: Role, org_type: Vec<u8>, metadata: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let acct = <AccountStorage<T>>::try_get(&who);
			match acct {
				Ok(_) => Err(Error::<T>::AlreadyExist)?,
				Err(_) => <AccountStorage<T>>::insert(
					&who,
					Account{
						id: who.clone(),
						name,
						role,
						status: Status::default(),
						org_type,
						metadata,
					},
				),
			}
			Self::deposit_event(Event::AccountRegisted);
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000)]
		pub fn update(origin: OriginFor<T>, name: Vec<u8>, role: Role, org_type: Vec<u8>, metadata: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let acct = <AccountStorage<T>>::try_get(&who);
			match acct {
				Err(_) => Err(Error::<T>::AccountNotRegistered)?,
				Ok(_) => {
					<AccountStorage<T>>::remove(&who);
					<AccountStorage<T>>::insert(
						&who,
						Account{
							id: who.clone(),
							name,
							role,
							status: Status::default(),
							org_type,
							metadata,
						},
					);
				},
			}
			Self::deposit_event(Event::AccountUpdated);
			Ok(())
		}
	}
}
