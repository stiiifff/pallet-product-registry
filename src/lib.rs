//! # Substrate Enterprise Sample - Product Registry example pallet

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use core::result::Result;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, sp_runtime::RuntimeDebug,
    sp_std::prelude::*,
};
use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// The product ID would typically be a GS1 GTIN (Global Trade Item Number),
// or ASIN (Amazon Standard Identification Number), or similar,
// a numeric or alpha-numeric code with a well-defined data structure.
pub const PRODUCT_ID_MAX_LENGTH: usize = 14;
pub type ProductId = Vec<u8>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Product<AccountId, Moment> {
    pub id: ProductId,
    pub owner: AccountId,
    pub creation: Moment,
}

pub trait Trait: system::Trait + timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ProductRegistry {
        pub Products get(fn product_by_id): map hasher(blake2_128_concat) ProductId => Option<Product<T::AccountId, T::Moment>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        ProductCreated(AccountId, AccountId, ProductId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        ProductIdMissing,
        ProductIdTooLong,
        ProductIdExists,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn create_product(origin, owner: T::AccountId, product_id: ProductId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            // TODO: assuming owner is a DID representing an organization,
            //       validate tx sender is owner or delegate of organization.

            // Validate product ID
            Self::validate_product_id(&product_id)?;

            // Check product doesn't exist yet (1 DB read)
            Self::validate_new_product(&product_id)?;

            // TODO: if organization has an attribute w/ GS1 Company prefix,
            //       additional validation could be applied to the product ID
            //       to ensure it is valid (same company prefix as org).

            // Create a product instance
            let product = Self::new_product()
                .identified_by(product_id.clone())
                .owned_by(owner.clone())
                .created_on(<timestamp::Module<T>>::now())
                .build();

            // Add product (1 DB write)
            <Products<T>>::insert(&product_id, product);

            Self::deposit_event(RawEvent::ProductCreated(who, owner, product_id));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    // Helper methods
    fn new_product() -> ProductBuilder<T::AccountId, T::Moment> {
        ProductBuilder::<T::AccountId, T::Moment>::default()
    }

    pub fn validate_product_id(id: &[u8]) -> Result<(), Error<T>> {
        // Basic product ID validation
        ensure!(!id.is_empty(), Error::<T>::ProductIdMissing);
        ensure!(
            id.len() <= PRODUCT_ID_MAX_LENGTH,
            Error::<T>::ProductIdTooLong
        );
        Ok(())
    }

    pub fn validate_new_product(id: &[u8]) -> Result<(), Error<T>> {
        // Product existence check
        ensure!(
            !<Products<T>>::contains_key(id),
            Error::<T>::ProductIdExists
        );
        Ok(())
    }
}

#[derive(Default)]
pub struct ProductBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{
    id: ProductId,
    owner: AccountId,
    creation: Moment,
}

impl<AccountId, Moment> ProductBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{
    pub fn identified_by(mut self, id: ProductId) -> Self {
        self.id = id;
        self
    }

    pub fn owned_by(mut self, owner: AccountId) -> Self {
        self.owner = owner;
        self
    }

    pub fn created_on(mut self, creation: Moment) -> Self {
        self.creation = creation;
        self
    }

    pub fn build(self) -> Product<AccountId, Moment> {
        Product::<AccountId, Moment> {
            id: self.id,
            owner: self.owner,
            creation: self.creation,
        }
    }
}
