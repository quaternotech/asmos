// Copyright 2023 Quaterno LLC
//
// Author: Mansoor Ahmed Memon <mansoorahmed.one@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::error::Error;

#[cfg(target_arch = "x86_64")]
fn bake_configurations() -> Result<(), Box<dyn Error>> {
    let arch = env::var("TARGET")?;
    let profile = env::var("PROFILE")?;
    let flavor = "vanilla";

    let work_dir = format!("cfg/konfigurator/{}/{}/{}", arch, flavor, profile);
    let out_dir = env::var("OUT_DIR")?;

    konfigurator::bake(work_dir, out_dir.clone(), true)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());

    println!("cargo:rerun-if-changed=cfg");

    bake_configurations()?;

    Ok(())
}
