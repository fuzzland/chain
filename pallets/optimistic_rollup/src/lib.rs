#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
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
	use frame_support::traits::{Currency, LockableCurrency, ReservableCurrency};
	use frame_system::pallet_prelude::*;
	pub use frame_support::sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedSub, Convert, Dispatchable,
			SaturatedConversion, Saturating, StaticLookup, UniqueSaturatedFrom, UniqueSaturatedInto,
			Zero, TrailingZeroInput
		},
		Perbill, Percent, RuntimeDebug
	};
	use frame_support::traits::ExistenceRequirement::KeepAlive;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: ReservableCurrency<Self::AccountId> + LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
	}

	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


	#[pallet::storage]
	#[pallet::getter(fn staked_value)]
	pub type StakedValue<T: Config> = StorageMap<
		_,
		Blake2_128Concat, T::AccountId,
		BalanceOf<T>,
		ValueQuery
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn stake(origin: OriginFor<T>,
			 #[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let zero_account = T::AccountId::decode(&mut TrailingZeroInput::new(&[][..]))
				.expect("infinite input; qed");

			T::Currency::transfer(&who, &zero_account, amount, KeepAlive)?;

			<StakedValue<T>>::mutate(who, |bal| {
				*bal = bal.saturating_add(amount);
			});
			Ok(())
		}

	}
}
