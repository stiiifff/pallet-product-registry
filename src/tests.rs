// Tests to be written here

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, dispatch};

pub fn store_test_product<T: Trait>(id: ProductId, owner: T::AccountId, creation: T::Moment) {
    Products::<T>::insert(
        id.clone(),
        Product {
            id,
            owner,
            creation,
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
        let owner = account_key(TEST_ORGANIZATION);
        let product_id = String::from(TEST_PRODUCT_ID).into_bytes();
        let now = 42;
        Timestamp::set_timestamp(now);

        let result = ProductRegistry::create_product(
            Origin::signed(sender),
            owner.clone(),
            product_id.clone(),
        );

        assert_ok!(result);

        assert_eq!(
            ProductRegistry::product_by_id(&product_id),
            Some(Product {
                id: product_id,
                owner: owner,
                creation: now
            })
        );
    });
}

#[test]
fn create_product_with_invalid_sender() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ProductRegistry::create_product(Origin::NONE, account_key(TEST_ORGANIZATION), vec!()),
            dispatch::DispatchError::BadOrigin
        );
    });
}

#[test]
fn create_product_with_missing_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ProductRegistry::create_product(
                Origin::signed(account_key(TEST_SENDER)),
                account_key(TEST_ORGANIZATION),
                vec!()
            ),
            Error::<Test>::ProductIdMissing
        );
    });
}

#[test]
fn create_product_with_long_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ProductRegistry::create_product(
                Origin::signed(account_key(TEST_SENDER)),
                account_key(TEST_ORGANIZATION),
                String::from(LONG_VALUE).into_bytes()
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
            ProductRegistry::create_product(
                Origin::signed(account_key(TEST_SENDER)),
                account_key(TEST_ORGANIZATION),
                existing_product
            ),
            Error::<Test>::ProductIdExists
        );
    })
}
