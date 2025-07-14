//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

const MDBOOK_EXE: &str = env!("CARGO_BIN_FILE_MDBOOK");
const PREPROCESSOR_EXE: &str = env!("CARGO_BIN_FILE_X_PREPROCESSOR");

fn main() -> ! {
    let mut cmd = Command::new(MDBOOK_EXE);

    cmd.env(
        "MDBOOK_PREPROCESSOR__X_PREPROCESSOR__COMMAND",
        PREPROCESSOR_EXE,
    );

    for arg in env::args().skip(1) {
        cmd.arg(arg);
    }

    let err = cmd.exec();
    panic!("{err}")
}
