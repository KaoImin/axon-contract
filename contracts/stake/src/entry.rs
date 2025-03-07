// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::vec::Vec;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    high_level::{load_cell_type_hash, load_script, load_witness_args, QueryIter},
};

use protocol::{reader as axon, Cursor};
use util::{error::Error, helper::*};

pub enum MODE {
    UPDATE,
    BURN,
    ADMIN,
    COMPANION,
}

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // extract stake_args
    let stake_args: axon::StakeLockArgs = Cursor::from(args.to_vec()).into();
    let admin_identity = stake_args.admin_identity();
    let type_id_hash = stake_args.type_id_hash();
    let node_identity = stake_args.node_identity();

    // identify contract mode by witness
    let mode = match load_witness_args(0, Source::GroupInput) {
        Ok(witness) => {
            let value = witness.input_type().to_opt();
            if value.is_none() || value.as_ref().unwrap().len() != 1 {
                return Err(Error::BadWitnessInputType);
            }
            if value.unwrap().raw_data().to_vec().first().unwrap() == &0 {
                if node_identity.is_none() {
                    MODE::ADMIN
                } else {
                    MODE::BURN
                }
            } else {
                if node_identity.is_none() {
                    return Err(Error::UnknownMode);
                }
                MODE::COMPANION
            }
        }
        Err(_) => {
            if node_identity.is_some() {
                return Err(Error::UnknownMode);
            }
            MODE::UPDATE
        }
    };

    // extract AT type_id from type_script
    let typeid_or_at_type_hash = {
        let type_hash = load_cell_type_hash(0, Source::GroupInput)?;
        if type_hash.is_none() {
            return Err(Error::TypeScriptEmpty);
        }
        type_hash.unwrap()
    };

    match mode {
        MODE::ADMIN => {
            debug!("admin mode");
            // check admin signature
            if !secp256k1::verify_signature(&admin_identity.content()) {
                return Err(Error::SignatureMismatch);
            }
            let input_stake_data =
                get_stake_data_by_type_hash(&typeid_or_at_type_hash, Source::Input)?;
            let output_stake_data =
                get_stake_data_by_type_hash(&typeid_or_at_type_hash, Source::Output)?;
            if input_stake_data.version() != output_stake_data.version()
                || input_stake_data.checkpoint_type_hash()
                    != output_stake_data.checkpoint_type_hash()
                || input_stake_data.sudt_type_hash() != output_stake_data.sudt_type_hash()
                || output_stake_data.quorum_size() > 160
            {
                return Err(Error::AdminModeError);
            }
        }
        MODE::BURN => {
            debug!("burn mode");
            // check admin signature
            if !secp256k1::verify_signature(&admin_identity.content()) {
                return Err(Error::SignatureMismatch);
            }
            let mut at_cell_count = 0;
            QueryIter::new(load_cell_type_hash, Source::Output).for_each(|type_hash| {
                if type_hash.unwrap_or([0u8; 32]) == typeid_or_at_type_hash {
                    at_cell_count += 1;
                }
            });
            if at_cell_count != 0 {
                return Err(Error::ATCellShouldEmpty);
            }
        }
        MODE::COMPANION => {
            debug!("companion mode");
            // check normal signature
            if !secp256k1::verify_signature(&node_identity.unwrap().content()) {
                return Err(Error::SignatureMismatch);
            }
            let mut find_type_hash = false;
            QueryIter::new(load_cell_type_hash, Source::Input).for_each(|type_hash| {
                if type_hash.unwrap_or([0u8; 32])[..] == type_id_hash[..] {
                    find_type_hash = true;
                }
            });
            if !find_type_hash {
                return Err(Error::CompanionModeError);
            }
        }
        MODE::UPDATE => {
            debug!("update mode");
            // check stake_data between input and output
            let input_stake_data =
                get_stake_data_by_type_hash(&typeid_or_at_type_hash, Source::Input)?;
            let output_stake_data =
                get_stake_data_by_type_hash(&typeid_or_at_type_hash, Source::Output)?;
            if input_stake_data.version() != output_stake_data.version()
                || input_stake_data.checkpoint_type_hash()
                    != output_stake_data.checkpoint_type_hash()
                || input_stake_data.sudt_type_hash() != output_stake_data.sudt_type_hash()
                || input_stake_data.quorum_size() != output_stake_data.quorum_size()
            {
                return Err(Error::StakeInfoMatchError);
            }

            // prepare transformed stake_infos data
            let input_stake_infos_set = stakeinfos_into_set(&input_stake_data.stake_infos())?;
            let output_stake_infos_set = stakeinfos_into_set(&output_stake_data.stake_infos())?;

            // get different stake_info between input not_apply stake_infos and output not_apply stake_infos
            let quorum = input_stake_data.quorum_size();
            let checkpoint =
                get_checkpoint_from_celldeps(&input_stake_data.checkpoint_type_hash())?;
            let era = bytes_to_u64(&checkpoint.era());
            let mut input_notapply_stake_infos =
                filter_stakeinfos(era + 1, quorum, &input_stake_infos_set, FILTER::NOTAPPLY)?;
            let output_notapply_stake_infos =
                filter_stakeinfos(era + 1, quorum, &output_stake_infos_set, FILTER::NOTAPPLY)?;
            debug!(
                "output_notapply = {}/{}, input_notapply = {}/{}",
                output_notapply_stake_infos.len(),
                output_stake_infos_set.len(),
                input_notapply_stake_infos.len(),
                input_stake_infos_set.len()
            );
            let node_stake_info = {
                if output_notapply_stake_infos.len() != input_notapply_stake_infos.len() + 1 {
                    return Err(Error::StakeInfoMatchError);
                }
                let diff_stake_infos = output_notapply_stake_infos
                    .symmetric_difference(&input_notapply_stake_infos)
                    .cloned()
                    .collect::<Vec<_>>();
                if diff_stake_infos.len() != 1 {
                    return Err(Error::StakeInfoMatchError);
                }
                diff_stake_infos.first().unwrap().clone()
            };
            debug!("insert_stake_amount = {}", node_stake_info.stake_amount);

            // put node_stake_info into not_apply set
            if node_stake_info.inauguration_era > era + 1 {
                if !input_notapply_stake_infos.insert(node_stake_info.clone()) {
                    return Err(Error::StakeInfoDumplicateError);
                }
            }

            // check valid stake_info subset from input is equal to output's
            let mut input_applied_stake_infos =
                filter_stakeinfos(era, quorum, &input_stake_infos_set, FILTER::APPLY)?;
            debug!(
                "input_applied = {}/{}",
                input_applied_stake_infos.len(),
                input_stake_infos_set.len()
            );
            let mut input_applying_stake_infos =
                filter_stakeinfos(era + 1, quorum, &input_stake_infos_set, FILTER::APPLY)?;
            debug!(
                "input_applying = {}/{}",
                input_applying_stake_infos.len(),
                input_stake_infos_set.len()
            );
            let valid_stake_infos = {
                input_notapply_stake_infos.append(&mut input_applied_stake_infos);
                input_notapply_stake_infos.append(&mut input_applying_stake_infos);
                input_notapply_stake_infos
            };
            if valid_stake_infos != output_stake_infos_set {
                return Err(Error::StakeInfoMatchError);
            }

            // get valid stake_amount on node_stake_info.identity
            let mut valid_stake_amount = 0;
            output_stake_infos_set.iter().for_each(|object| {
                if object.identity == node_stake_info.identity
                    && object.stake_amount > valid_stake_amount
                {
                    valid_stake_amount = object.stake_amount;
                }
            });

            // check valid stake_amount from stake AT cells in output
            let output_sudt = get_sudt_from_stake_at_cell(
                &script.code_hash().unpack(),
                &input_stake_data.sudt_type_hash(),
                &node_stake_info.identity,
                Source::Output,
            )?;
            debug!(
                "output_sudt = {}, valid_sudt = {}",
                output_sudt, valid_stake_amount
            );
            if output_sudt != valid_stake_amount {
                return Err(Error::InvaidStakeATAmount);
            }

            // check stake AT amount of inputs
            let input_sudt = get_sudt_from_stake_at_cell(
                &script.code_hash().unpack(),
                &input_stake_data.sudt_type_hash(),
                &node_stake_info.identity,
                Source::Input,
            )?;

            debug!("input_sudt = {}", input_sudt);
            // check if it's withdraw operation
            if input_sudt > output_sudt {
                // build withdrawal AT lock_script and search withdrawal AT cell
                let withdrawal_lock_hash = calc_withdrawal_lock_hash(
                    &checkpoint.withdrawal_lock_code_hash(),
                    admin_identity,
                    &input_stake_data.checkpoint_type_hash(),
                    &node_stake_info.identity,
                );
                let period = bytes_to_u64(&checkpoint.period());
                let unlock_period = bytes_to_u32(&checkpoint.unlock_period()) as u64;
                debug!("period = {}, unlock_period = {}", period, unlock_period);
                let total_sudt = get_withdrawal_total_sudt_amount(
                    &withdrawal_lock_hash,
                    &input_stake_data.sudt_type_hash(),
                    period + unlock_period,
                    Source::Output,
                )?;
                debug!("withdraw_sudt = {}", total_sudt);
                if input_sudt - output_sudt != total_sudt {
                    return Err(Error::WithdrawCellSudtMismatch);
                }
            }
        }
    }

    Ok(())
}
