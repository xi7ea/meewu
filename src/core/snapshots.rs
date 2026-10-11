/*
    snapshots.rs
    SPDX-License-Identifier: GPL-3.0-only
    --=================================--
    Author: xitea <xitea@proton.me>
*/

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use std::io::{self, Write};
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};

/* -------------------=============================------------------- *\
- Follows what I wish to be a neat implementation/PoC of Snapshot-as-Overlay
(what a fancy comment block)

For context: Modern Android root managers don't actually modify the rootfs for
modules that need it - but rather they make use of something called 'overlayFS'
overlayFS, as the name suggests, is a filesystem overlayed on top of original.
In this case, it's overlayed on top of the actual Android rootfs, which in any
case is EROFS in modern times and couldn't be mounted r/w by standard methods
anymore.

This allows not only for modification of files that normally you wouldn't be
able to, but also acts as a safety net. Overlayed filesystems are temporary,
and can be unmounted at a given moment, once you unmount it boom, there it is:
the rootfs, intact!

For macOS, we achieve a similar effect using APFS snapshots:
Instead of overlayFS, we create a bootable snapshot of the system volume, apply
our modifications to it, and then boot from that snapshot. To undo, simply boot
from the original snapshot.
\* -------------------=============================------------------- */