/* -*- coding: utf-8 -*-

Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Parser del Building Description Language (BDL) de DOE
//!
//! Definición de marco de hueco (NAME-FRAME)

use failure::Error;
use std::convert::TryFrom;

use crate::bdl::BdlBlock;

/// Marco de hueco (NAME-FRAME)
#[derive(Debug, Clone, Default)]
pub struct Frame {
    /// Nombre
    pub name: String,
    /// Grupo al que pertenece (biblioteca)
    pub group: String,
    /// Transmitancia térmica, U (W/m2K)
    pub conductivity: f32,
    /// Absortividad del marco, alpha (-)
    pub absorptivity: f32,
    /// Ancho del marco (m)
    pub width: f32,
}

impl TryFrom<BdlBlock> for Frame {
    type Error = Error;

    /// Conversión de bloque BDL a marco de hueco (NAME-FRAME)
    ///
    /// Conductividad en FRAME-CONDUCT (W/m2K)
    /// Absortividad(alpha) en FRAME-ABS (-)
    ///
    /// Ejemplo:
    /// ```text
    ///      "Marco PVC_1" = NAME-FRAME
    ///      GROUP         = "Marcos HULC2020"
    ///      FRAME-WIDTH   =            0.1
    ///      FRAME-CONDUCT =            1.3
    ///      FRAME-ABS     =            0.7
    ///      NAME_CALENER  = ""
    ///      LIBRARY       = NO
    ///      UTIL          =  NO
    ///      ..
    /// ```
    /// TODO: Propiedades no trasladadas: NAME-CALENER, LIBRRARY, UTIL
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let group = attrs.remove_str("GROUP")?;
        let conductivity = attrs.remove_f32("FRAME-CONDUCT")?;
        let absorptivity = attrs.remove_f32("FRAME-ABS")?;
        let width = attrs.remove_f32("FRAME-WIDTH")?;
        Ok(Self {
            name,
            group,
            conductivity,
            absorptivity,
            width,
        })
    }
}
