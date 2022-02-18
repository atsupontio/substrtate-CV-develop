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
	use frame_support::{pallet_prelude::*, transactional};
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::traits::Hash;
	use scale_info::TypeInfo;
	use sp_runtime::traits::StaticLookup;

	#[cfg(feature = "std")]
  	use frame_support::serde::{Deserialize, Serialize};

	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Human<T: Config> {
		pub id: AccountOf<T>,
		pub name: Option<u8>, // developing
		pub role: Role,
		pub role_confirm: RoleConfirm, // Set by Sys-man only
		pub org_type: Option<u8>, // developing
		pub sysman_level: Option<u8>, // Set by Sys-man only
		pub parent_sysman_ids: Option<T::Hash>, // Set by Sys-man only
		pub cvenkey: Option<u8>, // developing
		pub score: Option<u8>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct SysmanInfo<T: Config> {
		pub role: Role,
		pub role_confirm: RoleConfirm,
		pub sysman_level: u8,
		pub parent_syusman_ids: AccountOf<T>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum RoleConfirm {
		Yes,
		No
	}

	impl Default for RoleConfirm {
		fn default() -> Self {
			Self::No
		}
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Role {
		USER,
		ORG,
		SYSMAN,
	}

	impl Default for Role {
		fn default() -> Self {
			Self::USER
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The origin which may forcibly set or remove a name. Root can always do this.
		type ForceOrigin: EnsureOrigin<Self::Origin>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	
	#[pallet::storage]
	#[pallet::getter(fn participants)]
	// Stores a participants' information
	pub(super) type Participants<T: Config> = StorageMap<_, Twox64Concat, AccountOf<T>, Human<T>>;

	#[pallet::storage]
	#[pallet::getter(fn participant_cnt)]
	// keeps track of the number of all participants in existence
	pub(super) type ParticipantCnt<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn user_cnt)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// keeps track of the number of sysmans in existence
	pub(super) type UserCnt<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn users)]
	// Stores a sysman's information
	pub(super) type Users<T: Config> = StorageMap<_, Twox64Concat, AccountOf<T>, Human<T>>;

	#[pallet::storage]
	#[pallet::getter(fn org_cnt)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// keeps track of the number of sysmans in existence
	pub(super) type OrgCnt<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn orgs)]
	// Stores a sysman's information
	pub(super) type Orgs<T: Config> = StorageMap<_, Twox64Concat, AccountOf<T>, Human<T>>;

	#[pallet::storage]
	#[pallet::getter(fn sysman_cnt)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// keeps track of the number of sysmans in existence
	pub(super) type SysmanCnt<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sysmans)]
	// Stores a sysman's information
	pub(super) type Sysmans<T: Config> = StorageMap<_, Twox64Concat, AccountOf<T>, Human<T>>;

	// Our pallet's genesis configuration.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub sysman: Vec<(
			AccountOf<T>,
			Option<u8>, // developing
			Role,
			RoleConfirm, // Set by Sys-man only
			Option<u8>, // developing
			Option<u8>, // Set by Sys-man only
			Option<T::Hash>, // Set by Sys-man only
			Option<u8>, // developing
			Option<u8>,
		)>,
	}

	// Required to implement default for GenesisConfig.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { sysman: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a kitty from genesis config, we require the dna and gender to be supplied.
			for (acct, name, role, role_confirm, org_type, sysman_level, parent_sysman_ids, cvenkey, score) in &self.sysman {
				let _ = <Pallet<T>>::mint(acct, name.clone(), role.clone(), role_confirm.clone(), org_type.clone(), sysman_level.clone(), parent_sysman_ids.clone(), cvenkey.clone(), score.clone());
			}
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(T::AccountId, T::AccountId),

		///parameters. [AccountID]
		InformationCreated(T::AccountId),
		///parameters. [AccountID]
		InformationUpdated(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Handles checking whether the Sysman exists.
		SysmanNotExist,
		/// Handles checking whether the User exists.
		UserNotExist,
		/// Handles checking whether the Org exists.
		OrgNotExist,
		/// Handles checking that the account exists as participant.
		NotParticipant,
		/// Handles checking that the account exists as Sysman.
		NotSysman,
		/// Handles checking that the account exists as Org.
		NotOrg,
		/// Handles checking that the account exists as User.
		NotUser,
		/// Handles arithemtic overflow when incrementing the participant counter.
		ParticipantCntOverflow,
		/// Handles arithemtic overflow when incrementing the user counter.
		UserCntOverflow,
		/// Handles arithemtic overflow when incrementing the org counter.
		OrgCntOverflow,
		/// Handles arithemtic overflow when incrementing the sysman counter.
		SysmanCntOverflow,
		/// Handles checking whether the User's Role exists.
		NotExistRole,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		
		#[pallet::weight(10_000)]
		pub fn update_info(
			origin: OriginFor<T>,
			// id: T::AccountId,
			name: Option<u8>,
			org_type: Option<u8>,
			cvenkey: Option<u8>,
			score: Option<u8>,
			// owner: <T::Lookup as StaticLookup>::Source
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let origin = ensure_signed(origin)?;

			let participant = Self::participants(&origin);

			match participant {
				Some(a) => {Self::kill_store(&origin, &a);
					Self::mint(&origin, name, a.role, Default::default(), org_type, a.sysman_level, a.parent_sysman_ids,
					cvenkey, score);
					// Emit an event.
					Self::deposit_event(Event::InformationUpdated(origin));
				},
				None => { Self::mint(&origin, name, Default::default(), Default::default(), org_type, None, None,
					cvenkey, score);
					// Emit an event.
					Self::deposit_event(Event::InformationCreated(origin));
				}
			};
			
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn sudo_update_info(
			origin: OriginFor<T>,
			id: T::AccountId,
			name: Option<u8>,
			role: Role,
			role_confirm: RoleConfirm,
			org_type: Option<u8>,
			sysman_level: Option<u8>,
			parent_sysman_ids: Option<T::Hash>,
			cvenkey: Option<u8>,
			score: Option<u8>,
			// owner: <T::Lookup as StaticLookup>::Source
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			T::ForceOrigin::ensure_origin(origin)?;
			
			let ex_id = Self::mint(&id, name, role, role_confirm, org_type, sysman_level, parent_sysman_ids,
				cvenkey, score,
			)?;
			
			// Emit an event.
			// Self::deposit_event(Event::SudoSomethingStored(ex_id, id.clone()));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	}

	// helper function
	impl<T: Config> Pallet<T> {
		pub fn mint(
			id: &T::AccountId,
			name: Option<u8>,
			role: Role,
			role_confirm: RoleConfirm,
			org_type: Option<u8>,
			sysman_level: Option<u8>,
			parent_sysman_ids: Option<T::Hash>,
			cvenkey: Option<u8>,
			score: Option<u8>,
		) -> Result<(), Error<T>> {
			let user = Human::<T> {
				id: id.clone(),
				name: name,
				role: role.clone(),
				role_confirm: role_confirm,
				org_type: org_type,
				sysman_level: sysman_level,
				parent_sysman_ids: parent_sysman_ids,
				cvenkey: cvenkey,
				score: score,
			};

			Self::do_store(&id, &user)
		}

		pub fn is_participant(ex_id: &AccountOf<T>, acct: &Human<T>) -> Result<(), Error<T>> {
			match Self::participants(ex_id) {
				Some(participant) => Ok(()),
				None => Err(<Error<T>>::NotParticipant)
			}
		}

		pub fn is_sysman(acct: &Human<T>) -> Result<(), Error<T>> {
			match acct.role == Role::SYSMAN {
				True => Ok(()),
				False => Err(<Error<T>>::NotSysman)
			}
		}

		pub fn is_user(acct: &Human<T>) -> Result<(), Error<T>> {
			match acct.role == Role::USER {
				True => Ok(()),
				False => Err(<Error<T>>::NotUser)
			}
		}

		pub fn is_org(acct: &Human<T>) -> Result<(), Error<T>> {
			match acct.role == Role::ORG {
				True => Ok(()),
				False => Err(<Error<T>>::NotOrg)
			}
		}

		
		fn do_store(ex_id: &AccountOf<T>, user: &Human::<T>) -> Result<(), Error<T>> {

			Self::store_participant(ex_id, user);

			match user.role.clone() {
				Role::USER => Self::store_user(ex_id, user),
				Role::ORG => Self::store_org(ex_id, user),
				Role::SYSMAN => Self::store_sysman(ex_id, user),
				_ => Err(Error::<T>::NotExistRole),
			}

		}

		pub fn kill_store(ex_id: &AccountOf<T>, user: &Human::<T>) -> Result<(), Error<T>> {

			Self::kill_participant(ex_id);

			match user.role.clone() {
				Role::USER => Self::kill_user(ex_id),
				Role::ORG => Self::kill_org(ex_id),
				Role::SYSMAN => Self::kill_sysman(ex_id),
				_ => Err(Error::<T>::NotExistRole),
			}
		}

		fn kill_participant(prev_ex_id: &AccountOf<T>) -> Result<(), Error<T>> {
			let new_cnt = Self::participant_cnt().checked_sub(1)
        		.ok_or(<Error<T>>::ParticipantCntOverflow)?;
			
			<ParticipantCnt<T>>::put(new_cnt);

			// Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
			<Participants<T>>::remove(prev_ex_id);
			Ok(())
		}

		fn kill_user(prev_ex_id: &AccountOf<T>) -> Result<(), Error<T>> {
			let new_cnt = Self::user_cnt().checked_sub(1)
			.ok_or(<Error<T>>::UserCntOverflow)?;
		
			<UserCnt<T>>::put(new_cnt);

			// Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
			<Users<T>>::remove(prev_ex_id);
			Ok(())
		}

		fn kill_org(prev_ex_id: &AccountOf<T>) -> Result<(), Error<T>> {
			let new_cnt = Self::org_cnt().checked_sub(1)
        		.ok_or(<Error<T>>::OrgCntOverflow)?;
			
			<OrgCnt<T>>::put(new_cnt);

			// Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
			<Orgs<T>>::remove(prev_ex_id);
			Ok(())
		}

		fn kill_sysman(prev_ex_id: &AccountOf<T>) -> Result<(), Error<T>> {
			let new_cnt = Self::sysman_cnt().checked_sub(1)
        		.ok_or(<Error<T>>::SysmanCntOverflow)?;
			
			<SysmanCnt<T>>::put(new_cnt);

			// Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
			<Sysmans<T>>::remove(prev_ex_id);
			Ok(())
		}

		pub fn store_participant(
			ex_id: &AccountOf<T>,
			participant: &Human<T>,
		) -> Result<(), Error<T>> {
			let new_cnt = Self::participant_cnt().checked_add(1)
        		.ok_or(<Error<T>>::ParticipantCntOverflow)?;
			
			<ParticipantCnt<T>>::put(new_cnt);
			<Participants<T>>::insert(ex_id.clone(), participant.clone());
			Ok(())
		}

		pub fn store_user(
			ex_id: &AccountOf<T>,
			user: &Human<T>,
		) -> Result<(), Error<T>> {
			let new_cnt = Self::user_cnt().checked_add(1)
        		.ok_or(<Error<T>>::UserCntOverflow)?;
			
			<UserCnt<T>>::put(new_cnt);
			<Users<T>>::insert(ex_id.clone(), user.clone());
			Ok(())
		}

		pub fn store_org(
			org_id: &AccountOf<T>,
			org: &Human<T>,
		) -> Result<(), Error<T>> {
			let new_cnt = Self::org_cnt().checked_add(1)
        		.ok_or(<Error<T>>::OrgCntOverflow)?;
			
			<OrgCnt<T>>::put(new_cnt);
			<Orgs<T>>::insert(org_id.clone(), org.clone());
			Ok(())
		}

		pub fn store_sysman(
			sysman_id: &AccountOf<T>,
			sysman: &Human<T>,
		) -> Result<(), Error<T>> {
			let new_cnt = Self::sysman_cnt().checked_add(1)
        		.ok_or(<Error<T>>::SysmanCntOverflow)?;
			
			<SysmanCnt<T>>::put(new_cnt);
			<Sysmans<T>>::insert(sysman_id.clone(), sysman.clone());
			Ok(())
		}

	}
}
