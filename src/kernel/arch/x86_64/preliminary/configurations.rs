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

// This file is generated by the `Konfigurator` tool during the build process.
// It contains the configuration settings for the application.
//
// The macro `load_generated_config` is used to load the generated configuration file.
// The macro invocation triggers the inclusion of the generated file, `Konfigurator.rs`,
// located in the `OUT_DIR` environment variable path. Make sure that the generated file
// is correctly placed in the specified location during the build process.
macro_rules! load_generated_config {
    () => {
        include!(concat!(env!("OUT_DIR"), "/Konfigurator.rs"));
    };
}

load_generated_config!();
