// -*- coding: utf-8, vim: expandtab:ts=4 -*-

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use libferroxid::conversion::endian::u16_as_little_endian_array;


pub fn op_code_little_endian(op_code: u16) -> [u8; 2] {
    return u16_as_little_endian_array(op_code);
}
