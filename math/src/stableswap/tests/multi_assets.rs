const D_ITERATIONS: u8 = 128;
const Y_ITERATIONS: u8 = 64;

use crate::stableswap::types::AssetReserve;
use crate::stableswap::*;
use crate::types::Balance;
use sp_arithmetic::Permill;

const MAX_BALANCES: usize = 5;

#[test]
fn calculate_ann_should_work_when_correct_values_provided() {
	assert_eq!(calculate_ann(0, 100u128), Some(100u128));
	assert_eq!(calculate_ann(2, 1u128), Some(4u128));
	assert_eq!(calculate_ann(2, 10u128), Some(40u128));
	assert_eq!(calculate_ann(2, 100u128), Some(400u128));
}

#[test]
fn calculate_out_given_in_should_work_when_max_supported_nbr_of_balances_is_provided() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let idx_in: usize = 2;
	let idx_out: usize = 4;

	let amount_in: Balance = 2_000u128;

	let result = calculate_out_given_in::<D_ITERATIONS, Y_ITERATIONS>(&balances, idx_in, idx_out, amount_in, amp);

	assert!(result.is_some());
	let result = result.unwrap();

	assert_eq!(result, 1999u128);
}

#[test]
fn calculate_out_given_in_should_fail_when_asset_idx_is_incorrect() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let amount_in: Balance = 2_000u128;

	let result = calculate_out_given_in::<D_ITERATIONS, Y_ITERATIONS>(&balances, MAX_BALANCES, 1, amount_in, amp);

	assert!(result.is_none());

	let result = calculate_out_given_in::<D_ITERATIONS, Y_ITERATIONS>(&balances, 1, MAX_BALANCES, amount_in, amp);

	assert!(result.is_none());
}

#[test]
fn calculate_in_given_out_should_work_when_max_supported_nbr_of_balances_is_provided() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let idx_in: usize = 2;
	let idx_out: usize = 4;

	let amount_out: Balance = 2_000u128;

	let result = calculate_in_given_out::<D_ITERATIONS, Y_ITERATIONS>(&balances, idx_in, idx_out, amount_out, amp);

	assert!(result.is_some());
	let result = result.unwrap();

	assert_eq!(result, 2001u128);
}

#[test]
fn calculate_in_given_out_should_fail_when_asset_idx_is_incorrect() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let amount_out: Balance = 2_000u128;

	let result = calculate_in_given_out::<D_ITERATIONS, Y_ITERATIONS>(&balances, MAX_BALANCES, 1, amount_out, amp);

	assert!(result.is_none());

	let result = calculate_in_given_out::<D_ITERATIONS, Y_ITERATIONS>(&balances, 1, MAX_BALANCES, amount_out, amp);

	assert!(result.is_none());
}

#[test]
fn calculate_share_for_amount_should_return_correct_shares() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000_000_000_000_000, 12); MAX_BALANCES];

	let amount: Balance = 100_000_000_000_000;
	let issuance: Balance = 20_000_000_000_000_000_000_000;

	let result =
		calculate_shares_for_amount::<D_ITERATIONS>(&balances, 0, amount, amp, issuance, Permill::zero()).unwrap();

	assert_eq!(result, 40000002575489444434);

	let result = calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		result + 3000,
		0,
		issuance,
		amp,
		Permill::zero(),
	)
	.unwrap();
	assert_eq!(result, (amount, 0));
}

#[test]
fn calculate_share_for_amount_should_return_correct_shares_when_fee_applied() {
	let amp = 100_u128;

	let fee = Permill::from_float(0.001);

	let balances = [AssetReserve::new(10_000_000_000_000_000, 12); MAX_BALANCES];

	let amount: Balance = 100_000_000_000_000;
	let issuance: Balance = 20_000_000_000_000_000_000_000;

	let result = calculate_shares_for_amount::<D_ITERATIONS>(&balances, 0, amount, amp, issuance, fee).unwrap();

	assert_eq!(result, 40002502575973340332);

	let result =
		calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(&balances, result + 3000, 0, issuance, amp, fee)
			.unwrap();
	assert_eq!(result, (amount, 0));
}

#[test]
fn calculate_shares_should_work_when_correct_input_provided() {
	let amp = 100_u128;

	let initial_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];
	let mut updated_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];
	updated_balances[2].amount += 5000u128;

	let issuance: Balance = 100_000;

	let result = calculate_shares::<D_ITERATIONS>(&initial_balances, &updated_balances, amp, issuance);

	assert!(result.is_some());

	let result = result.unwrap();

	assert_eq!(result, 9999u128);
}

#[test]
fn calculate_shares_should_work_when_share_issuance_is_zero() {
	let amp = 100_u128;

	let initial_balances = [AssetReserve::new(0, 12); MAX_BALANCES];
	let mut updated_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];
	updated_balances[2].amount += 5000u128;

	let issuance: Balance = 0;

	let result = calculate_shares::<D_ITERATIONS>(&initial_balances, &updated_balances, amp, issuance);

	assert!(result.is_some());

	let result = result.unwrap();

	assert_eq!(result, 54999987033);
}

#[test]
fn calculate_shares_should_fail_when_balances_len_is_not_equal() {
	let amp = 100_u128;

	let initial_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES + 1];
	let mut updated_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];
	updated_balances[2].amount += 5000u128;

	let issuance: Balance = 100_000;

	let result = calculate_shares::<D_ITERATIONS>(&initial_balances, &updated_balances, amp, issuance);

	assert!(result.is_none());
}

#[test]
fn calculate_shares_should_fail_when_updated_balances_are_less() {
	let amp = 100_u128;

	let initial_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];
	let mut updated_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];
	updated_balances[2].amount -= 5000u128;

	let issuance: Balance = 100_000;

	let result = calculate_shares::<D_ITERATIONS>(&initial_balances, &updated_balances, amp, issuance);

	assert!(result.is_none());
}

#[test]
fn calculate_withdraw_one_asset_should_work_when_max_supported_nbr_of_balances_is_provided() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let asset_index: usize = 2;

	let shares_to_withdraw: Balance = 2_000u128;
	let issuance = 52000u128;

	let fee = Permill::from_percent(50);

	let result = calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		shares_to_withdraw,
		asset_index,
		issuance,
		amp,
		fee,
	);

	assert!(result.is_some());
	let result = result.unwrap();

	assert_eq!(result, (1442u128, 480u128));
}

#[test]
fn calculate_withdraw_one_asset_should_work_when_fee_is_zero() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let asset_index: usize = 2;

	let shares_to_withdraw: Balance = 2_000u128;
	let issuance = 52000u128;

	let fee = Permill::from_percent(0);

	let result = calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		shares_to_withdraw,
		asset_index,
		issuance,
		amp,
		fee,
	);

	assert!(result.is_some());
	let result = result.unwrap();

	assert_eq!(result, (1923, 0u128));
}

#[test]
fn calculate_withdraw_one_asset_should_work_when_fee_hundred_percent() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let asset_index: usize = 2;

	let shares_to_withdraw: Balance = 2_000u128;
	let issuance = 52000u128;

	let fee = Permill::from_percent(100);

	let result = calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		shares_to_withdraw,
		asset_index,
		issuance,
		amp,
		fee,
	);
	assert!(result.is_some());

	assert_eq!(result.unwrap(), (961, 961));
}

#[test]
fn calculate_withdraw_one_asset_should_fail_share_issuance_is_zero() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let asset_index: usize = 2;

	let shares_to_withdraw: Balance = 2_000u128;
	let issuance = 0u128;

	let fee = Permill::from_percent(0);

	let result = calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		shares_to_withdraw,
		asset_index,
		issuance,
		amp,
		fee,
	);

	assert!(result.is_none());
}

#[test]
fn calculate_withdraw_one_asset_should_fail_when_share_issuance_is_less_then_withdrawal() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let asset_index: usize = 2;

	let shares_to_withdraw: Balance = 2_000u128;
	let issuance = 1_000u128;

	let fee = Permill::from_percent(0);

	let result = calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		shares_to_withdraw,
		asset_index,
		issuance,
		amp,
		fee,
	);

	assert!(result.is_none());
}

#[test]
fn calculate_withdraw_one_asset_should_fail_asset_index_is_outside_boundaries() {
	let amp = 100_u128;

	let balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];

	let asset_index: usize = MAX_BALANCES;

	let shares_to_withdraw: Balance = 2_000u128;
	let issuance = 1_000u128;

	let fee = Permill::from_percent(0);

	let result = calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		shares_to_withdraw,
		asset_index,
		issuance,
		amp,
		fee,
	);

	assert!(result.is_none());
}

#[test]
fn calculate_withdraw_should_return_correct_amount_when_removing_provided_shares() {
	let amp = 100_u128;

	let fee = Permill::from_percent(0);

	let initial_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];
	let mut updated_balances = [AssetReserve::new(10_000, 12); MAX_BALANCES];
	updated_balances[2].amount += 5000u128;

	let issuance: Balance = 100_000;

	let result = calculate_shares::<D_ITERATIONS>(&initial_balances, &updated_balances, amp, issuance);
	let shares = result.unwrap();

	let result = calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
		&updated_balances,
		shares,
		2,
		issuance + shares,
		amp,
		fee,
	);

	assert!(result.is_some());

	let result = result.unwrap();

	assert_eq!(result, (4999u128, 0u128));
}

#[test]
fn calculate_out_given_in_with_fee_should_work_when_reserves_have_different_precision() {
	let amp = 1000_u128;

	let balances: [AssetReserve; 3] = [
		AssetReserve::new(1_000_000_000, 6),
		AssetReserve::new(3_000_000_000, 6),
		AssetReserve::new(5_000_000_000_000_000_000_000, 18),
	];

	let result = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_000,
		amp,
		Permill::from_percent(1),
	);
	assert_eq!(result.unwrap(), (990079130978583698, 10000799302813976));

	let result = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		2,
		1,
		1_000_000_000_000_000_000,
		amp,
		Permill::from_percent(1),
	);
	assert_eq!(result.unwrap(), (989920, 9999));
}

#[test]
fn calculate_out_given_in_with_zero_fee_should_work_when_reserves_have_different_precision() {
	let amp = 1000_u128;
	let balances: [AssetReserve; 3] = [
		AssetReserve::new(1_000_000_000, 6),
		AssetReserve::new(3_000_000_000, 6),
		AssetReserve::new(5_000_000_000_000_000_000_000, 18),
	];

	let result = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_000,
		amp,
		Permill::from_percent(0),
	);
	assert_eq!(result.unwrap(), (1000079930281397674, 0));

	let result = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		2,
		1,
		1_000_000_000_000_000_000,
		amp,
		Permill::from_percent(0),
	);
	assert_eq!(result.unwrap(), (999919, 0));
}

#[test]
fn calculate_in_given_out_with_fee_should_work_when_reserves_have_different_precision() {
	let amp = 1000_u128;
	let balances: [AssetReserve; 3] = [
		AssetReserve::new(1_000_000_000, 6),
		AssetReserve::new(3_000_000_000, 6),
		AssetReserve::new(5_000_000_000_000_000_000_000, 18),
	];

	let result = calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_000_000_000_000_000,
		amp,
		Permill::from_percent(1),
	);
	assert_eq!(result.unwrap(), (1_009_921, 10000));

	let result = calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		2,
		1,
		1_000_000,
		amp,
		Permill::from_percent(1),
	);
	assert_eq!(result.unwrap(), (1010080831907777026, 10000800315918585));
}

#[test]
fn calculate_in_given_out_with_zero_fee_should_work_when_reserves_have_different_precision() {
	let amp = 1000_u128;

	let balances: [AssetReserve; 3] = [
		AssetReserve::new(1_000_000_000, 6),
		AssetReserve::new(3_000_000_000, 6),
		AssetReserve::new(5_000_000_000_000_000_000_000, 18),
	];

	let result = calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_000_000_000_000_000,
		amp,
		Permill::from_percent(0),
	);
	assert_eq!(result.unwrap(), (999_921, 0));

	let result = calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		2,
		1,
		1_000_000,
		amp,
		Permill::from_percent(0),
	);
	assert_eq!(result.unwrap(), (1000080031591858441, 0));
}

#[test]
fn test_compare_precision_results_01() {
	let amp = 1000_u128;

	let balances: [AssetReserve; 3] = [
		AssetReserve::new(1_000_000_000_000_000_000_000, 18),
		AssetReserve::new(3_000_000_000_000_000_000_000, 18),
		AssetReserve::new(5_000_000_000_000_000_000_000, 18),
	];

	let d_before = calculate_d::<D_ITERATIONS>(&balances, amp).unwrap();
	let result = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_000_000_000_000_000,
		amp,
		Permill::from_percent(0),
	);
	let updated_reserves = [
		balances[0],
		AssetReserve::new(balances[1].amount + 1_000_000_000_000_000_000, balances[1].decimals),
		AssetReserve::new(balances[2].amount - result.unwrap().0, balances[2].decimals),
	];
	let d_after = calculate_d::<D_ITERATIONS>(&updated_reserves, amp).unwrap();
	assert!(d_after >= d_before);
	assert_eq!(result.unwrap(), (1_000_079_930_281_397_674, 0));

	let (amount_out, fee) = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		2,
		1,
		1_000_000_000_000_000_000,
		amp,
		Permill::from_percent(0),
	)
	.unwrap();
	assert_eq!((amount_out, fee), (999_919_974_816_739_669, 0));
	let updated_reserves = [
		balances[0],
		AssetReserve::new(balances[1].amount - amount_out, balances[1].decimals),
		AssetReserve::new(balances[2].amount + 1_000_000_000_000_000_000, balances[2].decimals),
	];
	let d_after = calculate_d::<D_ITERATIONS>(&updated_reserves, amp).unwrap();
	assert!(d_after >= d_before);
}

#[test]
fn test_compare_precision_results_02() {
	let amp = 1000_u128;

	let balances: [AssetReserve; 3] = [
		AssetReserve::new(1_000_000_000, 6),
		AssetReserve::new(3_000_000_000, 6),
		AssetReserve::new(5_000_000_000_000_000_000_000, 18),
	];

	let d_before = calculate_d::<D_ITERATIONS>(&balances, amp).unwrap();
	let result = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_000,
		amp,
		Permill::from_percent(0),
	);
	let updated_reserves = [
		balances[0],
		AssetReserve::new(balances[1].amount + 1_000_000, balances[1].decimals),
		AssetReserve::new(balances[2].amount - result.unwrap().0, balances[2].decimals),
	];
	let d_after = calculate_d::<D_ITERATIONS>(&updated_reserves, amp).unwrap();
	assert!(d_after >= d_before);
	assert_eq!(result.unwrap(), (1_000_079_930_281_397_674, 0));

	let (amount_out, fee) = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		2,
		1,
		1_000_000_000_000_000_000,
		amp,
		Permill::from_percent(0),
	)
	.unwrap();
	assert_eq!((amount_out, fee), (999_919, 0));
	let updated_reserves = [
		balances[0],
		AssetReserve::new(balances[1].amount - amount_out, balances[1].decimals),
		AssetReserve::new(balances[2].amount + 1_000_000_000_000_000_000, balances[2].decimals),
	];
	let d_after = calculate_d::<D_ITERATIONS>(&updated_reserves, amp).unwrap();
	assert!(d_after >= d_before);
}

#[test]
fn test_compare_precision_results_03() {
	let amp = 1000_u128;
	let balances: [AssetReserve; 3] = [
		AssetReserve::new(1_000_000_000_000_000_000_000, 18),
		AssetReserve::new(3_000_000_000_000_000_000_000, 18),
		AssetReserve::new(5_000_000_000_000_000_000_000, 18),
	];

	let d_before = calculate_d::<D_ITERATIONS>(&balances, amp).unwrap();
	let result = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_000_000_000_000_000,
		amp,
		Permill::from_percent(0),
	);
	let updated_reserves = [
		balances[0],
		AssetReserve::new(balances[1].amount + 1_000_000_000_000_000_000, balances[1].decimals),
		AssetReserve::new(balances[2].amount - result.unwrap().0, balances[2].decimals),
	];
	let d_after = calculate_d::<D_ITERATIONS>(&updated_reserves, amp).unwrap();
	assert!(d_after >= d_before);
	assert_eq!(result.unwrap(), (1_000_079_930_281_397_674, 0));

	let (amount_in, fee) = calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_079_930_281_397_674,
		amp,
		Permill::from_percent(0),
	)
	.unwrap();
	assert_eq!((amount_in, fee), (1000000000000000000, 0));
	let updated_reserves = [
		balances[0],
		AssetReserve::new(balances[1].amount + amount_in, balances[1].decimals),
		AssetReserve::new(balances[2].amount - 1_000_079_930_281_397_674, balances[2].decimals),
	];

	let d_after = calculate_d::<D_ITERATIONS>(&updated_reserves, amp).unwrap();
	assert!(d_after >= d_before);
}

#[test]
fn test_compare_precision_results_04() {
	let amp = 1000_u128;

	let balances: [AssetReserve; 3] = [
		AssetReserve::new(1_000_000_000, 6),
		AssetReserve::new(3_000_000_000, 6),
		AssetReserve::new(5_000_000_000_000_000_000_000, 18),
	];

	let d_before = calculate_d::<D_ITERATIONS>(&balances, amp).unwrap();
	let result = calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1_000_000,
		amp,
		Permill::from_percent(0),
	);
	let updated_reserves = [
		balances[0],
		AssetReserve::new(balances[1].amount + 1_000_000, balances[1].decimals),
		AssetReserve::new(balances[2].amount - result.unwrap().0, balances[2].decimals),
	];
	let d_after = calculate_d::<D_ITERATIONS>(&updated_reserves, amp).unwrap();
	assert!(d_after >= d_before);
	assert_eq!(result.unwrap(), (1000079930281397674, 0));

	let (amount_in, fee) = calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
		&balances,
		1,
		2,
		1000079930281397674,
		amp,
		Permill::from_percent(0),
	)
	.unwrap();
	assert_eq!((amount_in, fee), (1000001, 0));
	let updated_reserves = [
		balances[0],
		AssetReserve::new(balances[1].amount + amount_in, balances[1].decimals),
		AssetReserve::new(balances[2].amount - 1000079930281397674, balances[2].decimals),
	];
	let d_after = calculate_d::<D_ITERATIONS>(&updated_reserves, amp).unwrap();
	assert!(d_after >= d_before);
}
