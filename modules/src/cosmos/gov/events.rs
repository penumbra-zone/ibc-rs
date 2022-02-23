//! Event data structures for the Governance module of the Cosmos SDK.

use core::fmt;

use serde_derive::{Deserialize, Serialize};
use tendermint::abci::tag::Tag;
use tendermint::abci::Event as AbciEvent;

use crate::core::ics02_client::height::Height;
use crate::events::{IbcEvent, IbcEventType};

pub fn try_from_tx(event: &AbciEvent) -> Option<IbcEvent> {
    match event.type_str.parse().ok()? {
        IbcEventType::SubmitProposal => {
            SubmitProposal::try_from_event_tags(&event.attributes).map(IbcEvent::SubmitProposal)
        }
        _ => None,
    }
}

/// Data of a "submit_proposal" event carrying a "proposal_id" tag.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubmitProposal {
    pub height: Height,
    pub proposal_id: u64,
}

impl fmt::Display for SubmitProposal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{}", self.proposal_id)
    }
}

impl SubmitProposal {
    fn try_from_event_tags(tags: &[Tag]) -> Option<Self> {
        let mut proposal_id = None;
        for tag in tags {
            let key = tag.key.as_ref();
            let value = tag.value.as_ref();
            if key == "proposal_id" {
                proposal_id = Some(value.parse().ok()?);
            } else {
                continue;
            }
        }
        proposal_id.map(|proposal_id| Self {
            proposal_id,
            height: Default::default(),
        })
    }

    pub fn set_height(&mut self, height: Height) {
        self.height = height;
    }
}
