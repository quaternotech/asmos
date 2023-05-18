// MIT License
//
// Copyright (c) 2023 Mansoor Ahmed Memon.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
