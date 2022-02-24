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
	use frame_support::{pallet_prelude::*, Twox64Concat};
	use frame_system::pallet_prelude::*;
	use pallet_utils::{TypeID, WhoAndWhen, UnixEpoch, String};

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(bounds(), skip_type_params(T))]
	pub struct Item<T: Config> {
		item_id: TypeID,
		user_id: T::AccountId,
		created: WhoAndWhen<T>,
		org_date: Option<UnixEpoch>,
		exp_date: Option<UnixEpoch>,
		certificate_id: Option<TypeID>,
		score: u32,
		metadata: String,
	}

	impl <T: Config> Item<T> {
		pub fn new(
			id: TypeID,
			user_id: T::AccountId,
			created_by: T::AccountId,
			org_date: Option<UnixEpoch>,
			exp_date: Option<UnixEpoch>,
			certificate_id: Option<TypeID>,
			score: u32,
			metadata: String,
		) -> Self {
			Item {
				item_id: id,
				user_id,
				created: WhoAndWhen::<T>::new(created_by),
				org_date,
				exp_date,
				certificate_id,
				score,
				metadata
			}
		}
	} 

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum Status {
		Pending,
		Allow,
		Deny,
	}
	impl Default for Status {
		fn default() -> Self {
			Self::Pending
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_utils::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn item_by_id)]
	pub type ItemById<T> = StorageMap<_, Twox64Concat, TypeID, Item<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn item_by_account_id)]
	pub type ItemByAccountId<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<TypeID>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn item_id)]
	pub type ItemId<T> = StorageValue<_, TypeID, ValueQuery>;



	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		CreateSuceed(TypeID),
		RevokeSuceed(TypeID),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ItemNotFound,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000)]
		pub fn create_item(
			origin: OriginFor<T>,
			id: TypeID,
			user_id: T::AccountId,
			created_by: T::AccountId,
			org_date: Option<UnixEpoch>,
			exp_date: Option<UnixEpoch>,
			certificate_id: Option<TypeID>,
			score: u32,
			metadata: String,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let id = Self::item_id();
			let new_item: Item<T> = Item::new(id, user_id, who.clone(), org_date, exp_date, certificate_id, 0, metadata);

			<ItemById<T>>::insert(id, new_item);
			<ItemByAccountId<T>>::mutate(who, |x| x.push(id));
			<ItemId<T>>::mutate(|n| *n + 1);

			Self::deposit_event(Event::CreateSuceed(id));

			Ok(())

		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000)]
		pub fn revoke_error(origin: OriginFor<T>, item_id: TypeID, account_id: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let idx = Self::item_by_account_id(&account_id).iter().position(|x| *x == item_id);
			ensure!(idx != None, Error::<T>::ItemNotFound);
			if let Some(iid) = idx {
				<ItemByAccountId<T>>::mutate(&who, |x| x.swap_remove(iid));
			}
			<ItemById<T>>::remove(&item_id);
			Self::deposit_event(Event::RevokeSuceed(item_id));
			Ok(())
		}

		#[pallet::wight(10_000)]
		pub fn set_status_item(origin: OriginFor<T>, status: Status) {
			
		}
	}
}
