// Copyright (c) 2018-2022  Ministerio de Fomento
//                          Instituto de Ciencias de la Construcción Eduardo Torroja (IETcc-CSIC)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Author(s): Rafael Villar Burke <pachi@ietcc.csic.es>,
//            Daniel Jiménez González <dani@ietcc.csic.es>,
//            Marta Sorribes Gil <msorribes@ietcc.csic.es>

//! Vectores energéticos

use std::fmt;
use std::str;

use anyhow::{bail, Error};
use serde::{Deserialize, Serialize};

/// Vector energético (energy carrier).
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all="UPPERCASE")]
pub enum Carrier {
    /// Environment thermal energy (from heat pumps and other)
    Eambiente,
    /// Biofuel
    Biocarburante,
    /// Biomass
    Biomasa,
    /// Densified biomass (pellets)
    BiomasaDensificada,
    /// Coal
    Carbon,
    /// Electricity
    Electricidad,
    /// Natural gas
    GasNatural,
    /// Diesel oil
    Gasoleo,
    /// LPG - Liquefied petroleum gas
    Glp,
    /// Generic energy carrier 1
    Red1,
    /// Generic energy carrier 2
    Red2,
    /// Thermal energy from solar collectors
    Termosolar,
}

impl str::FromStr for Carrier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Carrier, Self::Err> {
        use Carrier::*;

        match s {
            "EAMBIENTE" => Ok(Eambiente),
            "BIOCARBURANTE" => Ok(Biocarburante),
            "BIOMASA" => Ok(Biomasa),
            "BIOMASADENSIFICADA" => Ok(BiomasaDensificada),
            "CARBON" => Ok(Carbon),
            "ELECTRICIDAD" => Ok(Electricidad),
            "GASNATURAL" => Ok(GasNatural),
            "GASOLEO" => Ok(Gasoleo),
            "GLP" => Ok(Glp),
            "RED1" => Ok(Red1),
            "RED2" => Ok(Red2),
            "TERMOSOLAR" => Ok(Termosolar),
            _ => bail!("Vector energético desconocido: {}", s),
        }
    }
}

impl std::fmt::Display for Carrier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
