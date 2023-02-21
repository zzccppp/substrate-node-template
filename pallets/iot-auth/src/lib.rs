#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::Randomness;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// type Currency: Currency<Self::AccountId>;
		type CollectionRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		#[pallet::constant]
		type MaximumOwned: Get<u32>;

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct IotDevice<T: Config> {
		pub unique_id: [u8; 16],
		pub owner: T::AccountId,
		// May Add More Later
		// 设备的公钥
		// 设备的信息
		// 设备的硬件ID
	}

	#[pallet::storage]
	pub(super) type IotDeviceCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub(super) type IotDeviceMap<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], IotDevice<T>>;

	#[pallet::storage]
	pub(super) type OwnerOfCollectibles<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<[u8; 16], T::MaximumOwned>,
		ValueQuery,
	>;

	#[pallet::error]
	pub enum Error<T> {
		/// Each collectible must have a unique identifier
		DuplicateDevices,
		/// An account can't exceed the `MaximumOwned` constant
		MaximumDevicesOwned,
		/// The total supply of collectibles can't exceed the u64 limit
		BoundsOverflow,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new collectible was successfully created.
		CollectibleCreated { collectible: [u8; 16], owner: T::AccountId },
	}

	impl<T: Config> Pallet<T> {
		fn gen_unique_id() -> [u8; 16] {
			// Create randomness
			let random = T::CollectionRandomness::random(&b"unique_id"[..]).0;

			// Create randomness payload. Multiple collectibles can be generated in the same block,
			// retaining uniqueness.
			let unique_payload = (
				random,
				frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
				frame_system::Pallet::<T>::block_number(),
			);

			// Turns into a byte array
			let encoded_payload = unique_payload.encode();
			let hash = frame_support::Hashable::blake2_128(&encoded_payload);

			hash
		}

		// Function to mint a collectible
		pub fn mint(owner: &T::AccountId, unique_id: [u8; 16]) -> Result<[u8; 16], DispatchError> {
			// Create a new object
			let collectible = IotDevice::<T> { unique_id, owner: owner.clone() };

			// Check if the collectible exists in the storage map
			ensure!(
				!IotDeviceMap::<T>::contains_key(&collectible.unique_id),
				Error::<T>::DuplicateDevices
			);

			// Check that a new collectible can be created
			let count = IotDeviceCount::<T>::get();
			let new_count = count.checked_add(1).ok_or(Error::<T>::BoundsOverflow)?;

			// Append collectible to OwnerOfCollectibles map
			OwnerOfCollectibles::<T>::try_append(&owner, collectible.unique_id)
				.map_err(|_| Error::<T>::MaximumDevicesOwned)?;

			// Write new collectible to storage and update the count
			IotDeviceMap::<T>::insert(collectible.unique_id, collectible);
			IotDeviceCount::<T>::put(new_count);

			// Deposit the "Collectiblereated" event.
			Self::deposit_event(Event::CollectibleCreated {
				collectible: unique_id,
				owner: owner.clone(),
			});

			// Returns the unique_id of the new collectible if this succeeds
			Ok(unique_id)
		}
	}

	// Pallet callable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Create a new unique devices
		#[pallet::weight(0)]
		pub fn create_devices(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;

			// Generate the unique_id and color using a helper function
			let collectible_gen_unique_id = Self::gen_unique_id();

			// Write new collectible to storage by calling helper function
			Self::mint(&sender, collectible_gen_unique_id)?;

			Ok(())
		}
	}
}
