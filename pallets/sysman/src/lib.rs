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
	// use frame_support::{pallet_prelude::{self::*, ValueQuery}, StorageMap};
	use frame_support::{pallet_prelude::*, Twox64Concat, dispatch::DispatchResultWithPostInfo, ensure};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use pallet_utils::{Role, Status};
	use sp_std::{str, vec, vec::Vec};

	// #[cfg(feature = "std")]
  	use serde::{Serialize, Deserialize};
	// use serde_json::{json, Value};

	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(bounds(), skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct SysManAccount<T: Config> {
		// pub id: AccountOf<T>,
		pub role: Role,
		pub status: Status,
		pub level: Option<u8>,
		pub parent: Option<AccountOf<T>>,
		pub children: Option<Vec<AccountOf<T>>>,
		pub metadata: Vec<u8>,
	}

	pub enum OperationType {
		SYS,
		ORG,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn org_storage)]
	// Stores a sysman's information
	pub(super) type OrgStorage<T: Config> = StorageMap<_, Twox64Concat, AccountOf<T>, SysManAccount<T>>;

	#[pallet::storage]
	#[pallet::getter(fn org_revoked)]
	// Stores a sysman's information
	pub(super) type OrgRevoked<T: Config> = StorageMap<_, Twox64Concat, AccountOf<T>, SysManAccount<T>>;

	#[pallet::storage]
	#[pallet::getter(fn sysman_storage)]
	// Stores a sysman's information
	pub(super) type SysmanStorage<T: Config> = StorageMap<_, Twox64Concat, AccountOf<T>, SysManAccount<T>>;

	#[pallet::storage]
	#[pallet::getter(fn sysman_revoked)]
	// Stores a sysman's information
	pub(super) type SysmanRevoked<T: Config> = StorageMap<_, Twox64Concat, AccountOf<T>, SysManAccount<T>>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub sysman: Vec<(AccountOf<T>, SysManAccount<T>)>,
	}

	// Required to implement default for GenesisConfig.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { sysman: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a kitty from genesis config, we require the dna and gender to be supplied.
			for (account_id, account) in &self.sysman {
				let _ = <SysmanStorage<T>>::insert(account_id, account);
			}
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		Approved { target_id: T::AccountId, metadata: Vec<u8>, approver: T::AccountId },
		Revoked { target_id: T::AccountId, revoker: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Handles checking whether the Sysman exists.
		SysmanNotExist,
		/// Handles checking whether the User exists.
		UserNotExist,
		/// Handles checking whether the Org exists.
		OrgNotExist,
		
		/// Handles checking whether the User's Role exists.
		NotExistRole,
		/// An account is already Sysman.
		AlreadySysman,
		AlreadyOrg,
		AlreadyRevoked,
		NoValidAuthorization,
		NotSysman,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.		

		#[pallet::weight(10_000)]
		pub fn approve_sysman(origin: OriginFor<T>, applicant: AccountOf<T>, metadata: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// check whether the executor of this function is system manager
			let authority = Self::get_account(&who, OperationType::SYS)?;

			// check whether the applicant is system-manager or already revoked
			ensure!(!<SysmanStorage<T>>::contains_key(&applicant), Error::<T>::AlreadySysman);
			ensure!(!<SysmanRevoked<T>>::contains_key(&applicant), Error::<T>::AlreadyRevoked);

			// create system manager account
			let sysman = Self::create_account(Role::SYSMAN, Default::default(), Some(authority.level.unwrap() + 1), Some(who.clone()), Some(vec![]), metadata.clone())?;

			<SysmanStorage<T>>::insert(&applicant, sysman);

			Self::deposit_event(Event::<T>::Approved {
				target_id: applicant,
				metadata,
				approver: who.clone(),
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn approve_org(origin: OriginFor<T>, applicant: AccountOf<T>, metadata: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// check whether the executor of this function is system manager
			let authority = Self::get_account(&who, OperationType::SYS)?;

			// check whether the applicant is system-manager or already revoked
			ensure!(!<OrgStorage<T>>::contains_key(&applicant), Error::<T>::AlreadyOrg);
			ensure!(!<OrgRevoked<T>>::contains_key(&applicant), Error::<T>::AlreadyRevoked);

			// create system manager account
			let org = Self::create_account(Role::ORG, Default::default(), None, None, None, metadata.clone())?;

			<OrgStorage<T>>::insert(&applicant, org);

			Self::deposit_event(Event::<T>::Approved {
				target_id: applicant,
				metadata,
				approver: who.clone(),
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_sysman(origin: OriginFor<T>, target: AccountOf<T>, metadata: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// check whether the executor of this function is system manager
			let authority = Self::get_account(&who, OperationType::SYS)?;

			let revoke_sysman = Self::get_account(&target, OperationType::SYS)?;

			// check whether the target is system-manager or already revoked
			ensure!(<SysmanStorage<T>>::contains_key(&target), Error::<T>::NotSysman);
			ensure!(!<SysmanRevoked<T>>::contains_key(&target), Error::<T>::AlreadyRevoked);

			// Ensure authority has higher hierarchical level than system manager to be revoked
			ensure!(
				authority.level.unwrap_or(0) < revoke_sysman.level.unwrap_or(0),
				Error::<T>::NoValidAuthorization
			);

			// eliminate account from storage
			<SysmanStorage<T>>::remove(&target);
			<SysmanRevoked<T>>::insert(&target, revoke_sysman);
			Self::deposit_event(Event::<T>::Revoked {
				target_id: target,
				revoker: who,
			});

			Ok(())
		}

		/* #[pallet::weight(10_000)]
		pub fn evaluate_org(origin: OriginFor<T>, ex_id: AccountOf<T>, score: u8) -> DispatchResult {
			ensure_signed(origin)?;

			let evaluate = Self::score(&ex_id);
			let new_cnt = Self::score(&ex_id).map(|(x, y)| (x.checked_add(score / y.checked_add(1)), y))
        		.ok_or(<Error<T>>::ParticipantCntOverflow)?;
			
			<ParticipantCnt<T>>::put(new_cnt);
			evaluate.unwrap().score = Some(score);
		} */

	}

	// helper function
	impl<T: Config> Pallet<T> {
		pub fn create_account(
			role: Role,
			status: Status,
			level: Option<u8>,
			parent: Option<AccountOf<T>>,
			children: Option<Vec<AccountOf<T>>>,
			metadata: Vec<u8>,
		) -> Result<SysManAccount<T>, Error<T>> {
			let sysman = SysManAccount::<T> {
				role: role.clone(),
				status,
				level,
				parent,
				children,
				metadata,
			};

			Ok(sysman)
		}			

		pub fn get_account(account_id: &AccountOf<T>, role: OperationType) -> Result<SysManAccount<T>, Error<T>> {
			let account = match role {
				OperationType::SYS => match <SysmanStorage<T>>::try_get(account_id) {
					Ok(acct) => match acct.role {
						Role::SYSMAN => acct,
						_ => Err(Error::<T>::NoValidAuthorization)?,
					}
					Err(_) => Err(Error::<T>::SysmanNotExist)?,
				},
				OperationType::ORG => match <OrgStorage<T>>::try_get(account_id) {
					Ok(acct) => match acct.role {
						Role::ORG => acct,
						_ => Err(Error::<T>::NoValidAuthorization)?,
					}
					Err(_) => Err(Error::<T>::OrgNotExist)?,
				},
			};
			Ok(account)
		}
	}
}
