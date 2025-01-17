// SPDX-License-Identifier: Apache-2.0

use clap::Args;

/// Calculate the cryptographic digest of a set of files.
#[derive(Args, Debug)]
pub struct Options {}

impl Options {
    pub fn execute(self) -> anyhow::Result<()> {
        unimplemented!()
    }
}
