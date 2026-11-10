/*
 * snapshots.rs
 * Copyright (c) 2026 xitea <xitea@duck.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-only
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