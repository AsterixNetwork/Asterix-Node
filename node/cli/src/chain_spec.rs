// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use sc_chain_spec::ChainSpecExtension;
use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use serde::{Serialize, Deserialize};
use asterix_node_runtime::{
	AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, ContractsConfig, CouncilConfig,
	DemocracyConfig, GrandpaConfig, ImOnlineConfig, SessionConfig, SessionKeys, StakerStatus,
	StakingConfig, ElectionsConfig, IndicesConfig, SocietyConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, wasm_binary_unwrap, MAX_NOMINATIONS,
};
use asterix_node_runtime::Block;
use asterix_node_runtime::constants::currency::*;
use sc_service::ChainType;
use hex_literal::hex;
use sc_telemetry::TelemetryEndpoints;
use grandpa_primitives::{AuthorityId as GrandpaId};
use sp_consensus_babe::{AuthorityId as BabeId};
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};

pub use node_primitives::{AccountId, Balance, Signature};
pub use asterix_node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "aster";
/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<
	GenesisConfig,
	Extensions,
>;

/// Celestial Testnet configuration
pub fn celestial_testnet() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/celestialTestnet.json")[..]) {
		Ok(spec) => spec,
		Err(e) => panic!("{}", e),
	}
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn celestial_testnet_config_genesis() -> GenesisConfig {


	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![
	(
// 5D32zNebD2nhdnmRUcUpbFeiRC32skqBqrqAnqt9fMjRhGXN
hex!["2af62649819eb78449730af0498b2efb75c24c91b0030f9c7a77c9966e33cd13"].into(),
// 5Eew6CEJVk3wvKYUPX8xFEe7yuEsy35JVnjKagXKYe6xX9qj
hex!["72935b8a6901e42cbee5d1ac4bf5658cc29d32223ddd0d237ad6967b561e5a36"].into(),
// 5GYPPonFJt296iJLkpDNxr9AxmYbz1nCvv6vtum3JKg4ebDs
hex!["c60da717d4292fe0de006f92d7d461c602ddeea8ed51f3d471c603b35c027eb0"].unchecked_into(),
// 5CartsfjrHtv8mZH5Nshz89cXhscYW2okPSXC3F3V3Q7NSuD
hex!["16ffc96ec89d5dc1cc4c8b90940ceae65ad2e40e37986bee01db1eeefa606944"].unchecked_into(),
// 5ERhGzoAdsmifgzjsaS7mv58uzyRbPT16WGbrw2tAsWXp3iz
hex!["687aaa86b58f66b92da15d950241383a1ef1b6fd3deecfe8a5c9debde868ae36"].unchecked_into(),
// 5DhdW15m658AcgVq2pXZWRKEPRZsnRHw63Np7SLTLBXhYQsq
hex!["4865a61395b2ff28c7d6747e81e20dae04fb7d28bf6aed5ac6c8baa0b05ab127"].unchecked_into(),
),

(
// 5EqSSNnU6q2h4rtaYjVg9djip1fKjzB7dTDyYYcpFDsUZkGr
hex!["7a969b0df06e5b1ac303d7acfa37f3d631e144d5ea83d81c68b31b8789609c7f"].into(),
// 5HmewNinZYngAJ7QMc1DERsEeHiy3XvbtnU1ZCK4DRtiwiNV
hex!["fc6867323d81bb5a195a2ce0afab83366c20f4e206ef3c2136e968a774d28b70"].into(),
// 5DsUVnzL7MmvzL1xvgcf13SRabfhDh2NZHZAWxmEZaxLua8f
hex!["4fe7cb4219fe211ddc5531e14b782763445050ac744b085e851d6ec787dda0e3"].unchecked_into(),
// 5ECorzB4LcnxZzxGrZiY5n9vBX2rdJthgWk9Xi6GJRnRxL1s
hex!["5ea6a5799fadaefdeab292f880fdcb070e82029c31c7811756341814bd90870d"].unchecked_into(),
// 5DAuQjdYn7c1EajBBhQV9iFFBekcy7BbB23FAaLoed23U2kD
hex!["30f698a9d6f3527a751d2bca2c5ef1798271dc282c196639cc9aeca2ae8ef17c"].unchecked_into(),
// 5GWVLuukTCHSknpZJrQP4RH5tTMS7VGYdvaEbjhV3wqyQLeB
hex!["c49b31142551592a1f352366a8e8fb70f436747c8ee93f4feb900e2fbf8a636f"].unchecked_into(),
),

(
// 5Dd9178iKx1RaFvbDdZLfE4nkdQC2t7NV1X5sLBSRH2rp4eE
hex!["44f8bbcea33cff49c602bdd00e5824be9729246b1e9f2ee1570428167c24ee30"].into(),
// 5DFXUKcz3LUwnjbaMv61isUhm9jSyDgPnmuhg2NDxhpw8Goq
hex!["347cf90f97cb327f3eca9787678b8c95f7146e92e492f25afcf1ba5d9dcddd52"].into(),
// 5Dmi6kSvoMH9wkx96B8e3g8U92utnfXHDBSwPc4eqkVJUJZN
hex!["4b821cbf7f3f9a4c2596363c97789ea0870e312006f8e9175ae2388cdd6610a1"].unchecked_into(),
// 5GE96L8SF8Lj45LoFmhQt5jUYJafedkm37fbz8NoN2j3fEXv
hex!["b82317c72abb2cedd74e03e0893e1606dcde02d097c26a30941b644c2fd7b350"].unchecked_into(),
// 5G49nEF2QgK4BP15N6fBUNTLMjnz49s3jspgCidQzxM2MUKQ
hex!["b084f45c682973dd3fe83fa074eb0b464c7f8843593ece0557b76f5ab5189d7a"].unchecked_into(),
// 5GN4UadC8FLpvpsawYDRzzsyzG3itLmvd8a8Amqm3xgPXMiv
hex!["be2d84184775dc2f95b6853d0a4d23a847d376ba754e3cfbbd40925c7d9a4a36"].unchecked_into(),
),
];


	let root_key: AccountId = hex![
		// 5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3
		"8af2f5f5dea00cef79bc4abd4f71c7e4ca2ebfe063b7b1b7fa918f69fa2c124e"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts), false)
}

/// Celestial testnet config.
pub fn celestial_testnet_config() -> ChainSpec {
	let data = r#"
			{
				"ss58Format": 42,
				"tokenDecimals": 18,
				"tokenSymbol": "ERX"
			}"#;
	let properties = serde_json::from_str(data).unwrap();
	let boot_nodes = vec![
		"/ip4/13.234.64.203/tcp/30333/p2p/12D3KooWQyBiFENzbPr9joiLoPa1StEbqdsrB5Exre2uib4MuEky"
			.parse()
			.unwrap(),
	];
	ChainSpec::from_genesis(
		"Celestial Testnet",
		"celestial_testnet",
		ChainType::Live,
		celestial_testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
			.expect("Celestial telemetry url is valid; qed")),
		Some(DEFAULT_PROTOCOL_ID),
		properties,
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	enable_println: bool,
) -> GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	initial_authorities.iter().map(|x| &x.0).chain(initial_nominators.iter()).for_each(|x| {
		if !endowed_accounts.contains(&x) {
			endowed_accounts.push(x.clone())
		}
	});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MAX_NOMINATIONS as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;

	GenesisConfig {
		frame_system: SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|x| (x, ENDOWMENT))
				.collect()
		},
		pallet_indices: IndicesConfig {
			indices: vec![],
		},
		pallet_session: SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		},
		pallet_staking: StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			.. Default::default()
		},
		pallet_democracy: DemocracyConfig::default(),
		pallet_elections_phragmen: ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		},
		pallet_collective_Instance1: CouncilConfig::default(),
		pallet_collective_Instance2: TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		},
		pallet_contracts: ContractsConfig {
			// println should only be enabled on development chains
			current_schedule: pallet_contracts::Schedule::default()
				.enable_println(enable_println),
		},
		pallet_sudo: SudoConfig {
			key: root_key,
		},
		pallet_babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(asterix_node_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		pallet_im_online: ImOnlineConfig {
			keys: vec![],
		},
		pallet_authority_discovery: AuthorityDiscoveryConfig {
			keys: vec![],
		},
		pallet_grandpa: GrandpaConfig {
			authorities: vec![],
		},
		pallet_membership_Instance1: Default::default(),
		pallet_treasury: Default::default(),
		pallet_society: SocietyConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			pot: 0,
			max_members: 999,
		},
		pallet_vesting: Default::default(),
		pallet_gilt: Default::default(),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
		],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	let data = r#"
			{
				"ss58Format": 42,
				"tokenDecimals": 18,
				"tokenSymbol": "ERX"
			}"#;
		let properties = serde_json::from_str(data).unwrap();
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		properties,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
			authority_keys_from_seed("Bob"),
		],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		false,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, new_light_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
			],
			vec![],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
			false,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sc_service_test::connectivity(
			integration_test_config_with_two_authorities(),
			|config| {
				let NewFullBase { task_manager, client, network, transaction_pool, .. }
					= new_full_base(config,|_, _| ())?;
				Ok(sc_service_test::TestNetComponents::new(task_manager, client, network, transaction_pool))
			},
			|config| {
				let (keep_alive, _, client, network, transaction_pool) = new_light_base(config)?;
				Ok(sc_service_test::TestNetComponents::new(keep_alive, client, network, transaction_pool))
			}
		);
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_celestial_test_net_chain_spec() {
		celestial_testnet_config().build_storage().unwrap();
	}
}
