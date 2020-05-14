#![feature(proc_macro_hygiene)]
use hdk_proc_macros::zome;
use serde_derive::{Deserialize, Serialize};
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::time::Iso8601,
    holochain_persistence_api::cas::content::Address,
};

pub mod note;
use crate::note::NoteEntry;
use crate::note::Note;

#[zome]
mod notes {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[entry_def]
     fn note_entry_def() -> ValidatingEntryType {
        note::definition()
    }

    #[zome_fn("hc_public")]
    fn create_note(base: String, note_input: NoteEntry) -> ZomeApiResult<Note> {
        note::handlers::create(base, note_input)
    }

    #[zome_fn("hc_public")]
    fn read_note(id: Address, created_at: Iso8601) -> ZomeApiResult<Note> {
        note::handlers::read(id, created_at)
    }

    #[zome_fn("hc_public")]
    fn update_note(id: Address, created_at: Iso8601, address: Address, note_input: NoteEntry) -> ZomeApiResult<Note> {
        note::handlers::update(id, created_at, address, note_input)
    }

	// No-one allowed to delete

    #[zome_fn("hc_public")]
    fn list_notes(base: String) -> ZomeApiResult<Vec<Note>> {
        note::handlers::list(base)
    }

    #[zome_fn("hc_public")]
    fn rebase_note(base_from: String, base_to: String, id: Address, created_at: Iso8601) -> ZomeApiResult<Address> {
        note::handlers::rebase(base_from, base_to, id, created_at)
    }

}