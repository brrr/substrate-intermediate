use super::*;
use crate::{mock::*, Error};

use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitty_works() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let mut account_id = 12;
        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        // assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id), *b"cutekitt"), sp_runtime::TokenError::NotExpendable
            );
        account_id = 1;
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"cutekitt"));
        assert_eq!(Balances::free_balance(account_id), BALANCE_RAW - KittyPrice::get());
        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        System::assert_last_event(Event::KittyCreated
            {
                who: 1,
                kitty_id: 0,
                kitty: KittiesModule::kitties(kitty_id).unwrap()
            }.into());

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);
        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id), *b"cutekitt"), Error::<Test>::InvalidKittyId
            );
    })
}

#[test]
fn breed_kitty_works() {
    new_test_ext().execute_with(|| {
        let account_id = 1;
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"cutekitt"));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"cutekitt"));
        assert_eq!(KittiesModule::next_kitty_id(), 2);
        assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), 0, 1, *b"cutekitt"));
        assert_eq!(Balances::free_balance(account_id),  BALANCE_RAW - 3 * KittyPrice::get());
        System::assert_last_event(Event::KittyBred
        {
            who: 1,
            kitty_id: 2,
            kitty: KittiesModule::kitties(2).unwrap()
        }.into());
        assert_eq!(KittiesModule::kitty_owner(2), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(2), Some((0, 1)));
        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), 1, 1, *b"cutekitt"), Error::<Test>::SameKittyId
            );
        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), 1, 9999, *b"cutekitt"), Error::<Test>::InvalidKittyId
            );
    })
}

#[test]
fn transfer_kitty_works() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;
        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"cutekitt"));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));
        assert_eq!(Balances::free_balance(account_id),  BALANCE_RAW -  KittyPrice::get());
        System::assert_last_event(Event::KittyTransferred
        {
            who: account_id,
            recipient: recipient,
            kitty_id: kitty_id
        }.into());
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));
        assert_noop!(
            KittiesModule::transfer(RuntimeOrigin::signed(9999), recipient, kitty_id), Error::<Test>::NotOwner
            );
    })
}

#[test]
fn sale_kitty_works() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"cutekitt"));
        assert_noop!(
            KittiesModule::sale(RuntimeOrigin::signed(9999), kitty_id), Error::<Test>::NotOwner
            );
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));
        assert_eq!(Balances::free_balance(account_id),  BALANCE_RAW -  KittyPrice::get());
        System::assert_last_event(Event::KittyOnSale
        {
            who: account_id,
            kitty_id: kitty_id
        }.into());
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_on_sale(kitty_id), Some(()));
        assert_noop!(
            KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id), Error::<Test>::AlreadyOnSale
            );
    })
}

#[test]
fn buy_kitty_works() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let buyer_id = 2;

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"cutekitt"));
        assert_noop!(
            KittiesModule::sale(RuntimeOrigin::signed(9999), kitty_id), Error::<Test>::NotOwner
            );
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));

        assert_eq!(Balances::free_balance(account_id),  BALANCE_RAW -  KittyPrice::get());
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(buyer_id), kitty_id));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(buyer_id));
        assert_eq!(Balances::free_balance(account_id),  BALANCE_RAW);
        assert_eq!(Balances::free_balance(buyer_id),  BALANCE_RAW -  KittyPrice::get());

        System::assert_last_event(Event::KittyBought
        {
            who: buyer_id,
            kitty_id: kitty_id
        }.into());

        assert_noop!(
            KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id), Error::<Test>::NotOnSale
            );
    })
}