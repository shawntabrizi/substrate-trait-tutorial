mod step1;
mod step2;
mod step3;
mod step4;
mod step5;

fn main() {
    println!("Run the tests!")
}

#[test]
fn test_step_1() {
    let mut balances = step1::BalancesModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.balance(1) == 100);
    assert!(balances.balance(2) == 200);

    assert!(balances.transfer(1, 2, 50).is_ok());

    assert!(balances.balance(1) == 50);
    assert!(balances.balance(2) == 250);

    assert!(balances.transfer(1, 2, 100).is_err());

    assert!(balances.balance(1) == 50);
    assert!(balances.balance(2) == 250);
}

#[test]
fn test_step_2() {
    let mut balances = step2::BalancesModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.balance(1) == 100);
    assert!(balances.balance(2) == 200);

    assert!(balances.transfer(1, 2, 50).is_ok());

    assert!(balances.balance(1) == 50);
    assert!(balances.balance(2) == 250);

    assert!(balances.transfer(1, 2, 100).is_err());

    assert!(balances.balance(1) == 50);
    assert!(balances.balance(2) == 250);
}

#[test]
fn test_step_3() {
    let mut voting = step3::VotingModule::new();
    voting.vote(1, 0, true);
    voting.vote(2, 0, false);

    assert!(voting.get_vote(1, 0) == true);
    assert!(voting.get_vote(2, 0) == false);

    assert!(voting.get_vote(1, 1) == false);
    assert!(voting.get_vote(2, 1) == false);
}

// // Account ID in step 2 and step 3 are defined differently!
// fn wont_work() {
//     let user_1 = 1;
//     let user_2 = 2;
//     let mut balances = step2::BalancesModule::new();
//     let mut voting = step3::VotingModule::new();

//     balances.set_balance(user_1, 100);
//     balances.set_balance(user_2, 200);

//     voting.vote(user_1, 0, true);
//     voting.vote(user_2, 0, false);
// }

#[test]
fn test_step_4() {
    type AccountId = u32;
    type Balance = u32;
    type VoteIndex = u8;

    let user_1: AccountId = 1;
    let user_2: AccountId = 2;

    let mut balances = step4::BalancesModule::<AccountId, Balance>::new();
    let mut voting = step4::VotingModule::<AccountId, VoteIndex>::new();

    balances.set_balance(user_1, 100);
    balances.set_balance(user_2, 200);

    voting.vote(user_1, 0, true);
    voting.vote(user_2, 0, false);
}

use std::cmp::Eq;
use std::hash::Hash;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Trait {
    type AccountId: Eq + Hash;
    type Balance: Zero + CheckedAdd + CheckedSub + Copy;
    type VoteIndex: Eq + Hash;
}

#[test]
fn test_step_5() {

    struct Runtime;
    impl Trait for Runtime {
        type AccountId = u32;
        type Balance = u64;
        type VoteIndex = u8;
    }

    let user_1: <Runtime as Trait>::AccountId = 1;
    let user_2: <Runtime as Trait>::AccountId = 2;

    let mut balances = step5::BalancesModule::<Runtime>::new();
    let mut voting = step5::VotingModule::<Runtime>::new();

    balances.set_balance(user_1, 100);
    balances.set_balance(user_2, 200);

    voting.vote(user_1, 0, true);
    voting.vote(user_2, 0, false);
}
