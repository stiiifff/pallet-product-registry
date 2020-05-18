//! # Substrate Enterprise Sample - Product Registry example pallet

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, sp_runtime::RuntimeDebug,
    sp_std::prelude::*,
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const PRODUCT_ID_MAX_LENGTH: usize = 14;

pub type ProductId = Vec<u8>;
pub type ValidationResult<R, T> = core::result::Result<R, Error<T>>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Product<Moment> {
    pub id: ProductId,
    pub creation: Moment,
}

pub trait Trait: system::Trait + timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ProductRegistry {
        pub Products get(fn product_by_id): map hasher(blake2_128_concat) ProductId => Option<Product<T::Moment>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        ProductCreated(AccountId, ProductId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        ProductIdMissing,
        ProductIdTooLong,
        ProductIdExists
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn create_product(origin, id: ProductId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            // Validate product ID
            Self::validate_product_id(&id)?;

            // Check product doesn't exist yet
            // Note: 1 DB read
            Self::validate_new_product(&id)?;

            // Add product (1 DB write)
            <Products<T>>::insert(&id, Product {
                id: id.clone(), creation: <timestamp::Module<T>>::now()
            });

            Self::deposit_event(RawEvent::ProductCreated(who, id));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    // Helpers
    pub fn validate_product_id(id: &[u8]) -> ValidationResult<(), T> {
        ensure!(id.len() > 0, Error::<T>::ProductIdMissing);
        ensure!(
            id.len() <= PRODUCT_ID_MAX_LENGTH,
            Error::<T>::ProductIdTooLong
        );
        Ok(())
    }

    pub fn validate_new_product(id: &[u8]) -> ValidationResult<(), T> {
        ensure!(
            !<Products<T>>::contains_key(id),
            Error::<T>::ProductIdExists
        );
        Ok(())
    }
}
