// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

// Tipos de sistemas del .ctehexml - GT

// Ver: https://energyplus.net/assets/nrel_custom/pdfs/pdfs_v9.5.0/EnergyPlusEssentials.pdf
// y esquema de E+ https://energyplus.readthedocs.io/en/latest/schema.html
// Ver: https://www.gbxml.org/schema_doc/6.01/GreenBuildingXML_Ver6.01.html#Link105
// https://doe2.com/Download/DOE-23/DOE23Vol2-Dictionary_50h.pdf
//
// Archivo BDLDialogsCALENER-GT_3_4.txt para referencias de variables por tipos de objeto
// Ver Manual Técnico GT

use std::str::FromStr;

use anyhow::{bail, Error};

use crate::bdl::BdlBlock;

use super::gt_types::*;

impl FromStr for PumpKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ONE_SPEED-PUMP" => Ok(Self::CaudalConstante),
            "TWO-SPEED-PUMP" => Ok(Self::DosVelocidades),
            "VAR-SPEED-PUMP" => Ok(Self::CaudalVariable),
            _ => bail!("Tipo de bomba hidráulica desconocido"),
        }
    }
}

impl From<BdlBlock> for GtPump {
    fn from(block: BdlBlock) -> Self {
        let eff = block.attrs.get_f32("MECH-EFF").unwrap_or(0.77)
            * block.attrs.get_f32("MOTOR-EFF").unwrap_or(0.80);
        Self {
            name: block.name.clone(),
            kind: block
                .attrs
                .get_str_or_default("CAP-CTRL")
                .parse()
                .unwrap_or_default(),
            flow: block.attrs.get_f32_or_default("C-C-FLOW"),
            head: block.attrs.get_f32_or_default("HEAD"),
            eff,
        }
    }
}

impl FromStr for CirculationLoopKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PIPE2" => Ok(Self::Pipe2),
            "DHW" => Ok(Self::Dhw),
            "CHW" => Ok(Self::Chw),
            "LAKE / WELL" => Ok(Self::LakeWell),
            "HW" => Ok(Self::Hw),
            "WLHP" => Ok(Self::Whlp),
            "CW" => Ok(Self::Cw),
            _ => bail!("Tipo de circuito hidráulico desconocido"),
        }
    }
}

impl From<BdlBlock> for GtCirculationLoop {
    fn from(block: BdlBlock) -> Self {
        use CirculationLoopKind::*;

        let kind = block
            .attrs
            .get_str_or_default("TYPE")
            .parse()
            .unwrap_or_default();
        let heat_setpoint_temp = block.attrs.get_f32("HEAT-SETPT-T").ok().or(match kind {
            Pipe2 | Hw => Some(80.0),
            Dhw => Some(50.0),
            Whlp | LakeWell => Some(20.0),
            _ => None,
        });
        let cool_setpoint_temp = block.attrs.get_f32("COOL-SETPT-T").ok().or(match kind {
            Pipe2 | Chw => Some(7.0),
            Dhw | Whlp | LakeWell => Some(30.0),
            _ => None,
        });
        Self {
            name: block.name.clone(),
            kind,
            dhw_flow: block.attrs.get_f32("C-C-FLOW").ok(),
            dhw_inlet_temp: block.attrs.get_f32("DHW-INLET-T").ok(),
            loop_pump: block.attrs.get_str("LOOP-PUMP").ok(),
            heat_setpoint_temp,
            cool_setpoint_temp,
        }
    }
}

impl FromStr for ChillerKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChillerKind::*;

        match s {
            "ELEC-HERM-REC" => Ok(ElecHermRec),
            "ELEC-HTREC" => Ok(ElecHeatRec),
            "ABSOR-1" => Ok(Absor1),
            "ABSOR-2" => Ok(Absor2),
            "GAS-ABSOR" => Ok(GasAbsor),
            "ENGINE" => Ok(Engine),
            "HEAT-PUMP" => Ok(HeatPump),
            "LOOP-TO-LOOP-HP" => Ok(LoopToLoopHeatPump),
            // No usados en GT
            // ELEC-OPEN-CENT y WATER-ECONOMIZER
            _ => bail!("Tipo de enfriadora desconocido"),
        }
    }
}

impl FromStr for CondenserKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CondenserKind::*;

        match s {
            "AIR-COOLED" => Ok(Air),
            "WATER-COOLED" => Ok(Water),
            // No usados en GT?
            "REMOTE-AIR-COOLED" => Ok(RemoteAir),
            "REMOTE-EVAP-COOLED" => Ok(RemoteEvap),
            _ => bail!("Tipo de condensación desconocido"),
        }
    }
}

impl From<BdlBlock> for GtChiller {
    fn from(block: BdlBlock) -> Self {
        let kind = block
            .attrs
            .get_str_or_default("TYPE")
            .parse()
            .unwrap_or_default();
        let condenser_kind = block
            .attrs
            .get_str_or_default("CONDENSER-TYPE")
            .parse()
            .unwrap_or_default();
        let fuel = match kind {
            ChillerKind::GasAbsor | ChillerKind::Engine => {
                block.attrs.get_str_or_default("FUEL-METER")
            }
            _ => "Electricidad".into(),
        };

        Self {
            name: block.name.clone(),
            kind,
            condenser_kind,
            cool_capacity: block.attrs.get_f32_or_default("C-C-CAPACITY"),
            eer: block.attrs.get_f32_or_default("C-NUM-OF-UNITS"),
            eer_th: block.attrs.get_f32("C-IPLV").ok(),
            heat_capacity: block.attrs.get_f32("C-DESIGN-KW").ok(),
            fuel,
            cop: block.attrs.get_f32("C-COP").ok(),
            chw_loop: block.attrs.get_str_or_default("CHW-LOOP"),
            cw_loop: block.attrs.get_str("CW-LOOP").ok(),
            hw_loop: block.attrs.get_str("HW-LOOP").ok(),
            htrec_loop: block.attrs.get_str("HTREC-LOOP").ok(),
        }
    }
}

impl From<BdlBlock> for GtBoiler {
    fn from(block: BdlBlock) -> Self {
        use BoilerKind::*;

        let kind = match block.attrs.get_str_or_default("TYPE").as_str() {
            "ELEC-HW-BOILER" => Electric,
            _ => match block.attrs.get_str_or_default("C-C-SUBTYPE").as_str() {
                "2" => LowTemp,
                "3" => Condensing,
                "4" => Biomass,
                "5" => Electric,
                // "1" => Conventional,
                _ => BoilerKind::default(),
            },
        };

        let fuel = block.attrs.get_str("FUEL-METER").unwrap_or(match kind {
            Electric => "Electricidad".into(),
            Biomass => "Biomasa".into(),
            _ => "Gas Natural".into(),
        });

        let eff = match kind {
            Electric => block.attrs.get_f32("C-AFUE").unwrap_or(0.98),
            // TODO: ver si los subtipos tienen rendimientos por defecto diferentes
            _ => block.attrs.get_f32("C-THERM-EFF-MAX").unwrap_or(0.85),
        };

        Self {
            name: block.name.clone(),
            kind,
            capacity: block.attrs.get_f32_or_default("C-C-CAPACITY"),
            eff,
            fuel,
            hw_loop: block.attrs.get_str_or_default("HW-LOOP"),
            hw_pump: block.attrs.get_str("HW-PUMP").ok(),
        }
    }
}

impl FromStr for DwHeaterKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DwHeaterKind::*;

        match s {
            "HEAT-PUMP" => Ok(HeatPump),
            "GAS" => Ok(Conventional),
            "ELEC" => Ok(Electric),
            _ => bail!("Tipo de calentador de ACS desconocido"),
        }
    }
}

impl From<BdlBlock> for GtDwHeater {
    fn from(block: BdlBlock) -> Self {
        use DwHeaterKind::*;

        let name = block.name.clone();
        let kind = block
            .attrs
            .get_str_or_default("TYPE")
            .parse()
            .unwrap_or_default();
        let capacity = block.attrs.get_f32_or_default("C-C-CAPACITY");

        let fuel = block.attrs.get_str("FUEL-METER").unwrap_or(match kind {
            Electric | HeatPump => "Electricidad".into(),
            _ => "Gas Natural".into(),
        });

        let eff = match kind {
            Electric => block.attrs.get_f32("C-STBY-LOSS-FRAC").unwrap_or(1.00),
            HeatPump => block.attrs.get_f32("C-STBY-LOSS-FRAC").unwrap_or(2.70),
            Conventional => block.attrs.get_f32("C-ENERGY-FACTOR").unwrap_or(0.80),
        };

        let has_tank = &block.attrs.get_str_or_default("C-CATEGORY") == "1";
        let dhw_tank = if has_tank {
            let volume = block
                .attrs
                .get_f32("TANK-VOLUME")
                .unwrap_or(65.0 * capacity);
            Some(GtHotWaterStorageTank {
                name: format!("Deposito - {}", name),
                volume,
                ua: block.attrs.get_f32("TANK-UA").unwrap_or(0.042 * volume),
            })
        } else {
            None
        };

        Self {
            name,
            kind,
            capacity,
            eff,
            fuel,
            dhw_loop: block.attrs.get_str_or_default("DHW-LOOP"),
            dhw_pump: block.attrs.get_str("DHW-PUMP").ok(),
            dhw_tank,
        }
    }
}

impl FromStr for HeatRejectionKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HeatRejectionKind::*;

        match s {
            "OPEN-TWR" => Ok(OpenTower),
            "FLUID-COOLER" => Ok(ClosedTower),
            _ => bail!("Tipo de condensación desconocido"),
        }
    }
}

impl From<BdlBlock> for GtHeatRejection {
    fn from(block: BdlBlock) -> Self {
        let name = block.name.clone();
        let kind = block
            .attrs
            .get_str_or_default("TYPE")
            .parse()
            .unwrap_or_default();
        let fuel = "Electricidad".into();

        Self {
            name,
            kind,
            fuel,
            capacity: block.attrs.get_f32_or_default("C-C-CAPACITY"),
            fan_kw: block.attrs.get_f32_or_default("FAN-KW/CELL"),
            number_of_cells: block
                .attrs
                .get_f32("NUMBER-OF-CELLS")
                .map(|v| v as u32)
                .unwrap_or(1),
            cw_loop: block.attrs.get_str_or_default("CW-LOOP"),
            cw_pump: block.attrs.get_str("CW-PUMP").ok(),
            spray_kw_cell: block.attrs.get_f32("SPRAY-KW/CELL").ok(),
        }
    }
}

impl From<BdlBlock> for GtElectricGenerator {
    fn from(block: BdlBlock) -> Self {
        let name = block.name.clone();
        let fuel = block
            .attrs
            .get_str("FUEL-METER")
            .unwrap_or_else(|_| "Gas Natural".into());

        Self {
            name,
            fuel,
            capacity: block.attrs.get_f32_or_default("CAPACITY"),
            eff: block.attrs.get_f32("C-C-HIR").unwrap_or(0.35),
            cw_loop: block.attrs.get_str("CW-LOOP").ok(),
            exh_loop: block.attrs.get_str("EXH-LOOP").ok(),
            jac_loop: block.attrs.get_str("JAC-LOOP").ok(),
        }
    }
}

impl FromStr for GroundLoopHxKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GroundLoopHxKind::*;

        match s {
            "LAKE/WELL" => Ok(LakeWell),
            "VERT-WELL-NEW" | "HORIZ-STRAIGHT-LOOP" | "HORIZ-SLINKY-LOOP" => Ok(Ground),
            _ => bail!("Tipo de condensación desconocido"),
        }
    }
}

impl From<BdlBlock> for GtGroundLoopHx {
    fn from(block: BdlBlock) -> Self {
        let name = block.name.clone();
        let kind = block
            .attrs
            .get_str_or_default("TYPE")
            .parse()
            .unwrap_or_default();

        Self {
            name,
            kind,
            circ_loop: block.attrs.get_str_or_default("CIRCULATION-LOOP"),
            loop_temp_sch: block.attrs.get_str_or_default("LOOP-TEMP-SCH"),
        }
    }
}

impl FromStr for GtSystemKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GtSystemKind::*;

        match s {
            "PSZ" => Ok(Psz),
            "PMZS" => Ok(Pmzs),
            "PVAVS" => Ok(Pvavs),
            "PVVT" => Ok(Pvvt),
            "SZRH" => Ok(Szrh),
            "VAVS" => Ok(Vavs),
            "RHFS" => Ok(Rhfs),
            "DDS" => Ok(Dds),
            "PTAC" => Ok(Ptac),
            "HP" => Ok(Hp),
            "FC" => Ok(Fc),
            "UVT" => Ok(Uvt),
            "UHT" => Ok(Uht),
            "FPH" => Ok(Fph),
            "EVAP-COOL" => Ok(EvapCool),
            "CBVAV" => Ok(Cbvav),
            _ => bail!("Tipo de sistema secundario desconocido"),
        }
    }
}

impl GtSystemKind {
    fn is_zone_system(self) -> bool {
        use GtSystemKind::*;
        self == Ptac || self == Hp || self == Fc || self == Uvt || self == Fph
    }
}

impl From<BdlBlock> for GtSystem {
    fn from(block: BdlBlock) -> Self {
        // # Identificación
        let name = block.name.clone();

        // # Tipo
        let kind: GtSystemKind = block
            .attrs
            .get_str_or_default("TYPE")
            .parse()
            .unwrap_or_default();

        // # Ventiladores

        let fan_schedule = block.attrs.get_str("FAN-SCHEDULE").ok();
        let supply_fan = if let Ok(supply_flow) = block.attrs.get_f32("C-C-SUPPLY-FLOW") {
            Some(Fan {
                flow: supply_flow,
                kw: if kind.is_zone_system() {
                    // Los sistemas de zona se definen por factor de transporte y no potencia
                    block.attrs.get_f32("C-C-SUP-KW/FLOW").unwrap_or(0.1) * supply_flow
                } else {
                    block.attrs.get_f32_or_default("C-C-SUPPLY-KW")
                },
            })
        } else {
            None
        };

        let return_fan = if let Ok(return_flow) = block.attrs.get_f32("RETURN-FLOW") {
            Some(Fan {
                flow: return_flow,
                kw: block.attrs.get_f32_or_default("C-C-RETURN-KW"),
            })
        } else {
            None
        };

        // # Calefacción y Refrigeración
        //
        // ## Potencias de calor y frío
        // Potencia de refrigeración, total y sensible:
        // Requerido en PSZ, PVAVS, PVVT, PTAC, HP, SZRH, VAVS, RHFS, DDS, FC, CBVAV
        // No usado en PMZS (default = 0)?, UVT, UHT, EVAP-COOL, FPH
        let cooling_cap = block.attrs.get_f32_or_default("C-C-COOL-CAP");
        let cooling_sh_cap = block
            .attrs
            .get_f32("C-C-COOL-SH-CAP")
            .unwrap_or(cooling_cap * 0.80);
        // Potencia de calefacción
        let heating_cap = block.attrs.get_f32_or_default("C-C-HEAT-CAP");

        // ## Refrigeración
        let cooling = if cooling_cap.abs() > f32::EPSILON {
            // TODO: convertir a coil / loop / autónomos
            Some(SysCooling {
                // Baterías:
                // Usado en todos: FPH, PSZ, PMZS (default=0)?, PVAVS, PVVT, PTAC, HP, SZRH, VAVS, RHFS, DDS, FC, UVT, UHT, EVAP-COOL, CBVAV
                chw_loop: block.attrs.get_str("CHW-LOOP").ok(),
                chw_coil_q: block.attrs.get_f32("C-C-CHW-COIL-Q").ok(),
                // Autónomos, DX, BdC, Cond. por agua, enf. evap...
                // Usado en PSZ, PVAVS, PVVT, PTAC, HP,
                // No usado en: FPH, PMZS, SZRH, VAVS, DDS, FC, UVT, UHT, EVAP-COOL, CBVAV
                eer: block.attrs.get_f32("C-C-EER").ok(),
                // TODO: Enfriamiento evaporativo y enfriadora de agua
            })
        } else {
            None
        };

        // ## Calefacción

        // Fuente de calor de las baterías principales a nivel de sistema
        // Usado en PSZ, PVAVS, PVVT, HP, SZRH, VAVS, RHFS, DDS, EVAP-COOL, CBVAV
        // No usado en PMZS, FPH, PTAC, FC, UVT, UHT
        let heating = build_heat_source("C-C-HEAT-SOURCE", &block).ok();

        // Fuente de calor a nivel de zona (en sistemas de aire centralizados)
        // Usado en FPH, PSZ, PVAVS, PVVT, PTAC, SZRH, VAVS, RHFS, FC, UVT, UHT, EVAP-COOL, CBVAV
        // No usado en PMZS, HP, DDS
        let zone_source = build_heat_source("C-C-ZONE-H-SOUR", &block).ok();

        // ## Precalentamiento
        let pre_heating = if let Ok(source) = build_heat_source("C-C-PREHEAT-SOURCE", &block) {
            // TODO: llevar a build_heat_source y eliminar tipo SysPreHeating
            Some(SysPreHeating {
                source,
                // Llevar al HeatSource
                capacity: block.attrs.get_f32_or_default("C-C-PREHEAT-CAP"),
                // Esto debería ir en el loop del HeatSource
                loop_name: block.attrs.get_str("PHW-LOOP").ok(),
            })
        } else {
            None
        };

        // ## Calefacción auxiliar (radiadores?)
        let aux_heating = if let Ok(source) = build_heat_source("C-C-BBRD-SOUR", &block) {
            // TODO: llevar a build_heat_source y eliminar SysAuxHeating
            Some(SysAuxHeating {
                source,
                // Los BBRD tienen su potencia de la zona
                // Llevar al HeatSource
                loop_name: block.attrs.get_str("BBRD-LOOP").ok(),
                dt: block.attrs.get_f32("BBRD-COIL-DT").ok(),
            })
        } else {
            None
        };

        // # Control

        let control = {
            // TODO: hay temperaturas por defecto según el tipo de secundario
            let min_supply_t = block.attrs.get_f32("MIN-SUPPLY-T").ok();
            let max_supply_t = block.attrs.get_f32("MAX-SUPPLY-T").ok();
            let heating_schedule = block.attrs.get_str("HEATING-SCHEDULE").ok();
            let cooling_schedule = block.attrs.get_str("COOLING-SCHEDULE").ok();

            if min_supply_t.is_none()
                && max_supply_t.is_none()
                && heating_schedule.is_none()
                && cooling_schedule.is_none()
            {
                None
            } else {
                Some(SysControl {
                    min_supply_t,
                    max_supply_t,
                    heating_schedule,
                    cooling_schedule,
                })
            }
        };

        // # Ventilación

        // ## Free cooling - Airside economizer
        let airside_economizer = if block
            .attrs
            .get_str("C-C-ENF-GRAT")
            .map(|v| v.trim() == "1")
            .unwrap_or_default()
        {
            if block.attrs.get_str_or_default("C-C-OA-CONTROL").trim() == "1" {
                Some(EconomizerControl::Enthalpy)
            } else {
                Some(EconomizerControl::Temperature)
            }
        } else {
            None
        };

        // ## Recuperación de calor del aire de expulsión - Energy recovery ventilators
        let exhaust_recovery = if block
            .attrs
            .get_str("RECOVER-EXHAUST")
            .map(|v| v.trim() == "YES")
            .unwrap_or_default()
        {
            Some(block.attrs.get_f32("ERV-SENSIBLE-EFF").unwrap_or(0.76))
        } else {
            None
        };

        Self {
            name,
            kind,
            control_zone: block.attrs.get_str("CONTROL-ZONE").ok(),

            // Air
            fan_schedule,
            supply_fan,
            return_fan,

            airside_economizer,
            exhaust_recovery,

            // Capacity
            heating_cap,
            cooling_cap,
            cooling_sh_cap,

            // Equipment
            cooling,
            heating,
            zone_source,
            pre_heating,
            aux_heating,

            control,
        }
    }
}

/// Convierte source a elementos concretos:
fn build_heat_source(source_id: &str, block: &BdlBlock) -> Result<HeatSource, Error> {
    use HeatSource::*;

    // Son números y tenemos que volver a convertirlos a cadenas!
    let source = block.attrs.get_f32_or_default(source_id).to_string();

    let is_zone_source = source_id.contains("-ZONE-");
    let is_preheat = source_id.contains("-PREHEAT-");
    let is_baseboard = source_id.contains("-BBRD-"); // TODO: resolver aquí calef. aux.

    let (heating_cap, cooling_cap, cooling_sh_cap) = if is_zone_source || is_baseboard {
        // Los sistemas de zona y los radiadores toman la potencia de la zona
        (None, None, None)
    } else if is_preheat {
        // Los sistemas de precalentamiento solo dan calor
        (
            Some(block.attrs.get_f32_or_default("C-C-PREHEAT-CAP")),
            None,
            None,
        )
    } else {
        // El resto podría dar calor o frío
        let cooling_cap = block.attrs.get_f32_or_default("C-C-COOL-CAP");
        (
            Some(block.attrs.get_f32_or_default("C-C-HEAT-CAP")),
            Some(cooling_cap),
            Some(
                block
                    .attrs
                    .get_f32("C-C-COOL-SH-CAP")
                    .unwrap_or(cooling_cap * 0.80),
            ),
        )
    };

    // Potencia de calefacción

    // 0=n/a, 1=eléctrica, 2=circuito agua caliente, 3=circuito ACS, 4=BdC eléctrica,
    // 5=BdC gas, 6=generador aire, 7=ninguna
    match source.as_str() {
        // Efecto joule
        // Deberíamos añadir vector electricidad?
        "1" => Ok(Electric { heating_cap }),
        // Circuito agua caliente, HwLoop
        "2" => {
            let w_loop = if is_zone_source {
                block
                    .attrs
                    .get_str("ZONE-HW-LOOP")
                    .or_else(|_| block.attrs.get_str("HW-LOOP"))
            } else if is_preheat {
                // TODO: el preheat solamente se puede introducir con un circuito de agua caliente?
                block
                    .attrs
                    .get_str("PHW-LOOP")
                    .or_else(|_| block.attrs.get_str("HW-LOOP"))
            } else {
                block.attrs.get_str("HW-LOOP")
            }
            .expect("No se encuentra circuito de agua en sistema");
            // let hw_coil_q = block.attrs.get_f32("C-C-HW-COIL-Q").ok();
            Ok(HotWaterLoop {
                heating_cap,
                w_loop,
            })
        }
        // Circuito agua caliente sanitaria, DhwLoop
        "3" => {
            let w_loop = block
                .attrs
                .get_str("DHW-LOOP")
                .or_else(|_| block.attrs.get_str("HW-LOOP"))
                .expect("No se encuentra circuito de acs en sistema");
            // let hw_coil_q = block.attrs.get_f32("C-C-HW-COIL-Q").ok();
            Ok(DhwLoop {
                heating_cap,
                w_loop,
            })
        }
        // Bomba de calor eléctrica, HeatPump
        // ¿En GT solamente dan calor o deberíamos leer C-C-EER (comprobar)?
        "4" => {
            let cop = block
                .attrs
                .get_f32("C-C-COP")
                .expect("Rendimiento COP no localizado para bomba de calor");
            Ok(HeatPump {
                heating_cap,
                cooling_cap,
                cooling_sh_cap,
                cop,
            })
        }
        // Bomba de calor a gas, GasHeatPump
        // ¿En GT solamente dan calor o deberíamos leer C-C-EER (comprobar)?
        "5" => {
            let cop = block
                .attrs
                .get_f32("C-C-COP")
                .expect("Rendimiento COP no localizado para bomba de calor a gas");
            Ok(GasHeatPump {
                heating_cap,
                cooling_cap,
                cooling_sh_cap,
                cop,
            })
        }
        // Generador de aire, Furnace
        "6" => {
            // p. 393 FURNACE-HIR, FURNACE-AUX
            let eff = block.attrs.get_f32_or_default("C-C-FURNACE-HIR");
            let aux_kw = block.attrs.get_f32_or_default("C-C-FURNACE-AUX");
            Ok(Furnace {
                heating_cap,
                eff,
                aux_kw,
            })
        }
        _ => bail!("Fuente de calor desconocida!"),
    }
}

impl FromStr for ZoneKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ZoneKind::*;

        match s {
            "CONDITIONED" => Ok(Conditioned),
            "PLENUM" => Ok(Plenum),
            "UNCONDITIONED" => Ok(Unconditioned),
            _ => bail!("Tipo de zona desconocido"),
        }
    }
}

impl From<BdlBlock> for GtZoneSystem {
    fn from(block: BdlBlock) -> Self {
        let name = block.name.clone();
        let kind = block
            .attrs
            .get_str_or_default("TYPE")
            .parse()
            .unwrap_or_default();

        let exhaust_fan = if block.attrs.get_str_or_default("C-C-PROP-ZR-1") == "1" {
            Some(Fan {
                flow: block.attrs.get_f32_or_default("C-C-EXH-FLOW"),
                kw: block.attrs.get_f32_or_default("C-C-EXH-KW"),
            })
        } else {
            None
        };

        let oa_flow = match block.attrs.get_str_or_default("C-C-OA-MET-DEF").as_str() {
            // Caudal total
            "1" => block
                .attrs
                .get_f32("C-C-OA-FLOW")
                .ok()
                .map(OutdoorAirFlow::Total),
            // Caudal por persona
            _ => block
                .attrs
                .get_f32("C-C-OA-FLOW/PER")
                .ok()
                .map(OutdoorAirFlow::PerPerson),
        };

        Self {
            name,
            kind,
            space: block.attrs.get_str_or_default("SPACE"),
            // Sistema asignado a la zona
            // El sistema se asigna tras la construcción
            system: None,
            // Termostatos
            heat_temp_sch: block.attrs.get_str("HEAT-TEMP-SCH").ok(),
            cool_temp_sch: block.attrs.get_str("COOL-TEMP-SCH").ok(),
            // impulsión de zona
            design_flow: block.attrs.get_f32("C-C-ASSIG-FLOW").ok(),
            // extracción de zona
            exhaust_fan,
            // aire exterior de zona
            oa_flow,
            // Equipamiento de zona
            cool_cap: block.attrs.get_f32("C-C-COOL-CAP").ok(),
            cool_sh_cap: block.attrs.get_f32("C-C-COOL-SH-CAP").ok(),
            heat_cap: block.attrs.get_f32("C-C-HEAT-CAP").ok(),
        }
    }
}
