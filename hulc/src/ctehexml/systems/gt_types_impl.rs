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
                .get_str("CAP-CTRL")
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
            flow: block.attrs.get_f32("C-C-FLOW").unwrap_or_default(),
            head: block.attrs.get_f32("HEAD").unwrap_or_default(),
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
            .get_str("TYPE")
            .unwrap_or_default()
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
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let condenser_kind = block
            .attrs
            .get_str("CONDENSER-TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let fuel = match kind {
            ChillerKind::GasAbsor | ChillerKind::Engine => {
                block.attrs.get_str("FUEL-METER").unwrap_or_default()
            }
            _ => "Electricidad".into(),
        };

        Self {
            name: block.name.clone(),
            kind,
            condenser_kind,
            cool_capacity: block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default(),
            eer: block.attrs.get_f32("C-NUM-OF-UNITS").unwrap_or_default(),
            eer_th: block.attrs.get_f32("C-IPLV").ok(),
            heat_capacity: block.attrs.get_f32("C-DESIGN-KW").ok(),
            fuel,
            cop: block.attrs.get_f32("C-COP").ok(),
            chw_loop: block.attrs.get_str("CHW-LOOP").unwrap_or_default(),
            cw_loop: block.attrs.get_str("CW-LOOP").ok(),
            hw_loop: block.attrs.get_str("HW-LOOP").ok(),
            htrec_loop: block.attrs.get_str("HTREC-LOOP").ok(),
        }
    }
}

impl From<BdlBlock> for GtBoiler {
    fn from(block: BdlBlock) -> Self {
        use BoilerKind::*;

        let kind = match block.attrs.get_str("TYPE").unwrap_or_default().as_str() {
            "ELEC-HW-BOILER" => Electric,
            _ => match block
                .attrs
                .get_str("C-C-SUBTYPE")
                .unwrap_or_default()
                .as_str()
            {
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
            capacity: block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default(),
            eff,
            fuel,
            hw_loop: block.attrs.get_str("HW-LOOP").unwrap_or_default(),
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
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let capacity = block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default();

        let fuel = block.attrs.get_str("FUEL-METER").unwrap_or(match kind {
            Electric | HeatPump => "Electricidad".into(),
            _ => "Gas Natural".into(),
        });

        let eff = match kind {
            Electric => block.attrs.get_f32("C-STBY-LOSS-FRAC").unwrap_or(1.00),
            HeatPump => block.attrs.get_f32("C-STBY-LOSS-FRAC").unwrap_or(2.70),
            Conventional => block.attrs.get_f32("C-ENERGY-FACTOR").unwrap_or(0.80),
        };

        let has_tank = &block.attrs.get_str("C-CATEGORY").unwrap_or_default() == "1";
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
            dhw_loop: block.attrs.get_str("DHW-LOOP").unwrap_or_default(),
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
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let fuel = "Electricidad".into();

        Self {
            name,
            kind,
            fuel,
            capacity: block.attrs.get_f32("C-C-CAPACITY").unwrap_or_default(),
            fan_kw: block.attrs.get_f32("FAN-KW/CELL").unwrap_or_default(),
            number_of_cells: block
                .attrs
                .get_f32("NUMBER-OF-CELLS")
                .map(|v| v as u32)
                .unwrap_or(1),
            cw_loop: block.attrs.get_str("CW-LOOP").unwrap_or_default(),
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
            capacity: block.attrs.get_f32("CAPACITY").unwrap_or_default(),
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
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();

        Self {
            name,
            kind,
            circ_loop: block.attrs.get_str("CIRCULATION-LOOP").unwrap_or_default(),
            loop_temp_sch: block.attrs.get_str("LOOP-TEMP-SCH").unwrap_or_default(),
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
        let name = block.name.clone();
        let kind: GtSystemKind = block
            .attrs
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();

        // Ventiladores

        let fans_schedule = block.attrs.get_str("FAN-SCHEDULE").ok();
        let supply_fan = if let Ok(supply_flow) = block.attrs.get_f32("C-C-SUPPLY-FLOW") {
            Some(Fan {
                flow: supply_flow,
                kw: if kind.is_zone_system() {
                    // Los sistemas de zona se definen por factor de transporte y no potencia
                    block.attrs.get_f32("C-C-SUP-KW/FLOW").unwrap_or(0.1) * supply_flow
                } else {
                    block.attrs.get_f32("C-C-SUPPLY-KW").unwrap_or_default()
                },
            })
        } else {
            None
        };

        let return_fan = if let Ok(return_flow) = block.attrs.get_f32("RETURN-FLOW") {
            Some(Fan {
                flow: return_flow,
                kw: block.attrs.get_f32("C-C-RETURN-KW").unwrap_or_default(),
            })
        } else {
            None
        };

        // Calefacción y refrigeración
        let cool_cap = block.attrs.get_f32("C-C-COOL-CAP").unwrap_or_default();
        let cool_sh_cap = block
            .attrs
            .get_f32("C-C-COOL-SH-CAP")
            .unwrap_or(cool_cap * 0.80);
        let cooling_coil = if cool_cap.abs() > f32::EPSILON {
            // TODO: convertir a coil / loop / autónomos
            Some(SysCoolingDetail {
                chw_loop: block.attrs.get_str("CHW-LOOP").ok(),
                chw_coil_q: block.attrs.get_f32("C-C-CHW-COIL-Q").ok(),
                // Autónomos, DX, BdC, Cond. por agua, enf. evap...
                eer: block.attrs.get_f32("C-C-EER").ok(),
            })
        } else {
            None
        };

        // Potencia de calefacción
        let heat_cap = block.attrs.get_f32("C-C-HEAT-CAP").unwrap_or_default();

        // Fuente de calor de las baterías principales a nivel de sistema
        let heat_source = build_heat_source("C-C-HEAT-SOURCE", &block).ok();

        // Fuente de calor a nivel de zona (en sistemas de aire centralizados)
        let zone_heat_source = build_heat_source("C-C-ZONE-H-SOUR", &block).ok();

        let pre_heating = if let Ok(source) = build_heat_source("C-C-PREHEAT-SOURCE", &block) {
            Some(SysPreHeating {
                source,
                capacity: block.attrs.get_f32("C-C-PREHEAT-CAP").unwrap_or_default(),
                // Esto debería ir en el loop del source?
                loop_name: block.attrs.get_str("PHW-LOOP").ok(),
            })
        } else {
            None
        };

        let aux_heating = if let Ok(source) = build_heat_source("C-C-BBRD-SOUR", &block) {
            Some(SysAuxHeating {
                source,
                // TODO: llevar a build_heat_source y hacer condicional con source_id
                loop_name: block.attrs.get_str("BBRD-LOOP").ok(),
                dt: block.attrs.get_f32("BBRD-COIL-DT").ok(),
            })
        } else {
            None
        };

        // Control

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

        // Sistemas de recuperación

        let recovery = {
            // Free cooling
            let free_cooling = if block
                .attrs
                .get_str("C-C-ENF-GRAT")
                .map(|v| v.trim() == "1")
                .unwrap_or_default()
            {
                if block
                    .attrs
                    .get_str("C-C-OA-CONTROL")
                    .unwrap_or_default()
                    .trim()
                    == "1"
                {
                    Some("Por entalpía".to_string())
                } else {
                    Some("Por temperatura".to_string())
                }
            } else {
                None
            };

            // Exhaust recovery
            let exhaust_recovery_eff = if block
                .attrs
                .get_str("RECOVER-EXHAUST")
                .map(|v| v.trim() == "YES")
                .unwrap_or_default()
            {
                Some(block.attrs.get_f32("ERV-SENSIBLE-EFF").unwrap_or(0.76))
            } else {
                None
            };

            if free_cooling.is_none() && exhaust_recovery_eff.is_none() {
                None
            } else {
                Some(SysRecovery {
                    free_cooling,
                    exhaust_recovery_eff,
                })
            }
        };

        Self {
            name,
            kind,
            control_zone: block.attrs.get_str("CONTROL-ZONE").ok(),
            fans_schedule,
            supply_fan,
            return_fan,
            cool_cap,
            cool_sh_cap,
            cool_detail: cooling_coil,
            heat_cap,
            heat_source,
            zone_heat_source,
            pre_heat: pre_heating,
            aux_heat: aux_heating,
            control,
            recovery,
        }
    }
}

/// Convierte source a elementos concretos:
fn build_heat_source(source_id: &str, block: &BdlBlock) -> Result<GtHeatSourceKind, Error> {
    use GtHeatSourceKind::*;

    // Son números!
    let source = block
        .attrs
        .get_f32(source_id)
        .unwrap_or_default()
        .to_string();

    // 0=n/a, 1=eléctrica, 2=circuito agua caliente, 3=circuito ACS, 4=BdC eléctrica,
    // 5=BdC gas, 6=generador aire, 7=ninguna
    match source.as_str() {
        // Efecto joule
        // Deberíamos añadir vector electricidad?
        "1" => Ok(Electric),
        // Circuito agua caliente, HwLoop
        "2" => {
            let w_loop = block
                .attrs
                .get_str("ZONE-HW-LOOP")
                .or_else(|_| block.attrs.get_str("HW-LOOP"))
                .expect("No se encuentra circuito de agua en sistema");
            // let hw_coil_q = block.attrs.get_f32("C-C-HW-COIL-Q").ok();
            Ok(HotWaterLoop { w_loop })
        }
        // Circuito agua caliente sanitaria, DhwLoop
        "3" => {
            let w_loop = block
                .attrs
                .get_str("DHW-LOOP")
                .or_else(|_| block.attrs.get_str("HW-LOOP"))
                .expect("No se encuentra circuito de acs en sistema");
            // let hw_coil_q = block.attrs.get_f32("C-C-HW-COIL-Q").ok();
            Ok(DhwLoop { w_loop })
        }
        // Bomba de calor eléctrica, HeatPump
        "4" => {
            let cop = block
                .attrs
                .get_f32("C-C-COP")
                .expect("Rendimiento COP no localizado para bomba de calor");
            Ok(HeatPump { cop })
        }
        // Bomba de calor a gas, GasHeatPump
        "5" => {
            let cop = block
                .attrs
                .get_f32("C-C-COP")
                .expect("Rendimiento COP no localizado para bomba de calor a gas");
            Ok(GasHeatPump { cop })
        }
        // Generador de aire, Furnace
        "6" => {
            // p. 393 FURNACE-HIR, FURNACE-AUX
            let eff = block.attrs.get_f32("C-C-FURNACE-HIR").unwrap_or_default();
            let aux_kw = block.attrs.get_f32("C-C-FURNACE-AUX").unwrap_or_default();
            Ok(Furnace { eff, aux_kw })
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
            .get_str("TYPE")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();

        let exhaust_fan = if block.attrs.get_str("C-C-PROP-ZR-1").unwrap_or_default() == "1" {
            Some(Fan {
                flow: block.attrs.get_f32("C-C-EXH-FLOW").unwrap_or_default(),
                kw: block.attrs.get_f32("C-C-EXH-KW").unwrap_or_default(),
            })
        } else {
            None
        };

        let oa_flow = match block
            .attrs
            .get_str("C-C-OA-MET-DEF")
            .unwrap_or_default()
            .as_str()
        {
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
            space: block.attrs.get_str("SPACE").unwrap_or_default(),
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
