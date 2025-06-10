/*
 * Copyright (C) 2023 Liquid Reply GmbH. All rights reserved.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


// tc.variables["WAMR_BUILD_INTERP"] = 1
// tc.variables["WAMR_BUILD_FAST_INTERP"] = 1
// tc.variables["WAMR_BUILD_INSTRUCTION_METERING"] = 1
// tc.variables["WAMR_BUILD_AOT"] = 0
// tc.variables["WAMR_BUILD_JIT"] = 0
// tc.variables["WAMR_BUILD_FAST_JIT"] = 0
// tc.variables["WAMR_DISABLE_HW_BOUND_CHECK"] = 1
// tc.variables["WAMR_DISABLE_STACK_HW_BOUND_CHECK"] = 1
// 
// 
// tc.variables["WAMR_BH_LOG"] = "wamr_log_to_rippled"