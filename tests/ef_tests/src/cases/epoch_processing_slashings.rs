use super::*;
use crate::case_result::compare_beacon_state_results_without_caches;
use serde_derive::Deserialize;
use state_processing::per_epoch_processing::{
    process_slashings::process_slashings, validator_statuses::ValidatorStatuses,
};
use types::{BeaconState, EthSpec};

#[derive(Debug, Clone, Deserialize)]
#[serde(bound = "E: EthSpec")]
pub struct EpochProcessingSlashings<E: EthSpec> {
    pub description: String,
    pub pre: BeaconState<E>,
    pub post: Option<BeaconState<E>>,
}

impl<E: EthSpec> YamlDecode for EpochProcessingSlashings<E> {
    fn yaml_decode(yaml: &str) -> Result<Self, Error> {
        Ok(serde_yaml::from_str(yaml).unwrap())
    }
}

impl<E: EthSpec> Case for EpochProcessingSlashings<E> {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn result(&self, _case_index: usize) -> Result<(), Error> {
        let mut state = self.pre.clone();
        let mut expected = self.post.clone();

        let spec = &E::default_spec();

        let mut result = (|| {
            // Processing requires the epoch cache.
            state.build_all_caches(spec)?;

            let mut validator_statuses = ValidatorStatuses::new(&state, spec)?;
            validator_statuses.process_attestations(&state, spec)?;
            process_slashings(
                &mut state,
                validator_statuses.total_balances.current_epoch,
                spec,
            )
            .map(|_| state)
        })();

        compare_beacon_state_results_without_caches(&mut result, &mut expected)
    }
}
