#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate bigint;

use pwasm_ethereum as eth;

use bigint::U256;
use pwasm_std::hash::Address;

use pwasm_test::ExternalBuilder;

test_with_external!(
	ExternalBuilder::new()
		.balance_of(Address::from([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]), 200000.into())
		.build(),
	check_balance {
		assert_eq!(
			U256::from(200000),
			eth::balance(&Address::from([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]))
		);
	}
);
