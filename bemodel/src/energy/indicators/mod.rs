// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Información energética relativa al modelo
//!
//! Cálculo de K, qsoljul, Fshobst, etc

mod types;
pub mod k;
pub mod n50;
pub mod qsoljul;

pub use types::EnergyIndicators;
pub use n50::N50Data;
pub use k::KData;
pub use qsoljul::QSolJulData;
