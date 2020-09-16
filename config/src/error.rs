use thiserror::Error;

#[derive(Debug,Error)]
pub enum Error {
	#[error("index out of range {} > {}", index, nodes)]
	IndexError{index:usize, nodes:usize},
	#[error("invalid SafetyRules backend {0}.")]
	InvalidSafetyRulesBackend(String),
	#[error("Missing configs only found {}", found)]
	MissingConfigs{found:usize},
	#[error("Missing full node network")]
	MissingFullNodeNetwork,
	#[error("MissingNetworkKeyPairs")]
	MissingNetworkKeyPairs,
	#[error("MissingSafetyRulesHost")]
	MissingSafetyRulesHost,
	#[error("MissingSafetyRulesToken")]
	MissingSafetyRulesToken,
	#[error("config does not contain a validator network")]
	MissingValidatorNetwork,
	#[error("Unable to find any configs")]
	NoConifg,
	#[error("network size should at least 1")]
	NoneZeroNetwork,
}