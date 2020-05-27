// Tests to be written here

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, dispatch};

pub fn store_test_product<T: Trait>(id: ProductId, owner: T::AccountId, registered: T::Moment) {
    Products::<T>::insert(
        id.clone(),
        Product {
            id,
            owner,
            registered,
            props: None,
        },
    );
}

const TEST_PRODUCT_ID: &str = "00012345600012";
const TEST_ORGANIZATION: &str = "Northwind";
const TEST_SENDER: &str = "Alice";
const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

#[test]
fn create_product_with_valid_args() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let id = String::from(TEST_PRODUCT_ID).into_bytes();
        let owner = account_key(TEST_ORGANIZATION);
        let now = 42;
        Timestamp::set_timestamp(now);

        let result = ProductRegistry::register_product(
            Origin::signed(sender),
            id.clone(),
            owner.clone(),
            None,
        );

        assert_ok!(result);

        assert_eq!(
            ProductRegistry::product_by_id(&id),
            Some(Product {
                id: id.clone(),
                owner: owner,
                registered: now,
                props: None
            })
        );

        assert_eq!(ProductRegistry::owner_of(&id), Some(owner));
    });
}

#[test]
fn create_product_with_invalid_sender() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ProductRegistry::register_product(
                Origin::NONE,
                vec!(),
                account_key(TEST_ORGANIZATION),
                None
            ),
            dispatch::DispatchError::BadOrigin
        );
    });
}

#[test]
fn create_product_with_missing_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ProductRegistry::register_product(
                Origin::signed(account_key(TEST_SENDER)),
                vec!(),
                account_key(TEST_ORGANIZATION),
                None
            ),
            Error::<Test>::ProductIdMissing
        );
    });
}

#[test]
fn create_product_with_long_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ProductRegistry::register_product(
                Origin::signed(account_key(TEST_SENDER)),
                String::from(LONG_VALUE).into_bytes(),
                account_key(TEST_ORGANIZATION),
                None
            ),
            Error::<Test>::ProductIdTooLong
        );
    })
}

#[test]
fn create_product_with_existing_id() {
    new_test_ext().execute_with(|| {
        let existing_product = String::from(TEST_PRODUCT_ID).into_bytes();
        let now = 42;
        store_test_product::<Test>(
            existing_product.clone(),
            account_key(TEST_ORGANIZATION),
            now,
        );

        assert_noop!(
            ProductRegistry::register_product(
                Origin::signed(account_key(TEST_SENDER)),
                existing_product,
                account_key(TEST_ORGANIZATION),
                None
            ),
            Error::<Test>::ProductIdExists
        );
    })
}
