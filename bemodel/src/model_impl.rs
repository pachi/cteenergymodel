// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Implementación de funciones de acceso e identificación de elementos y cálculos geométricos generales

use log::{info, warn};

use super::{BoundaryType, Model, Space, SpaceType, Tilt, Wall, WallCons, Window, WindowCons};
use crate::utils::fround2;

impl Model {
    /// Localiza espacio
    pub fn space_by_id<'a>(&'a self, spaceid: &'a str) -> Option<&'a Space> {
        self.spaces.iter().find(|s| s.id == spaceid)
    }

    /// Localiza espacio por nombre
    pub fn space_by_name<'a>(&'a self, spacename: &'a str) -> Option<&'a Space> {
        self.spaces.iter().find(|s| s.name == spacename)
    }

    /// Localiza espacio de opaco
    pub fn space_of_wall<'a>(&'a self, wall: &'a Wall) -> Option<&'a Space> {
        let maybespace = self.space_by_id(&wall.space);
        if maybespace.is_none() {
            warn!(
                "Muro {} ({}) con definición de espacio incorrecta {}",
                wall.id, wall.name, wall.space
            );
        }
        maybespace
    }

    /// Localiza opaco
    pub fn wall_by_id<'a>(&'a self, wallid: &'a str) -> Option<&'a Wall> {
        self.walls.iter().find(|w| w.id == wallid)
    }

    /// Localiza opaco por nombre
    pub fn wall_by_name<'a>(&'a self, wallname: &'a str) -> Option<&'a Wall> {
        self.walls.iter().find(|w| w.name == wallname)
    }

    /// Grosor de un elemento opaco
    pub fn wall_thickness(&self, wallid: &str) -> f32 {
        self.wall_by_id(wallid)
            .and_then(|w| self.wallcons_for_wall(&w).map(|c| c.thickness))
            .unwrap_or(0.0)
    }

    /// Localiza construcción de opaco
    pub fn wallcons_for_wall<'a>(&'a self, wall: &'a Wall) -> Option<&'a WallCons> {
        let maybecons = self.wallcons.iter().find(|wc| wc.id == wall.cons);
        if maybecons.is_none() {
            warn!(
                "Muro {} ({}) con definición de construcción incorrecta {}",
                wall.id, wall.name, wall.cons
            );
        };
        maybecons
    }

    /// Localiza muro de hueco
    pub fn wall_of_window<'a>(&'a self, window: &'a Window) -> Option<&'a Wall> {
        let maybewall = self.wall_by_id(&window.wall);
        if maybewall.is_none() {
            warn!(
                "Hueco {} ({}) con definición de muro incorrecta {}",
                window.id, window.name, window.wall
            );
        }
        maybewall
    }

    /// Localiza construcción de hueco
    pub fn wincons_of_window<'a>(&'a self, win: &'a Window) -> Option<&'a WindowCons> {
        let maybecons = self.wincons.iter().find(|wc| wc.id == win.cons);
        if maybecons.is_none() {
            warn!(
                "Hueco {}({}) con definición de construcción incorrecta {}",
                win.id, win.name, win.cons
            );
        }
        maybecons
    }

    /// Iterador de los huecos pertenecientes a un muro
    pub fn wincons_of_window_iter<'a>(&'a self, wallid: &'a str) -> impl Iterator<Item = &'a Window> {
        self.windows.iter().filter(move |w| w.wall == wallid)
    }

    /// Iterador de los cerramientos (incluyendo muros, suelos y techos) que delimitan un espacio
    pub fn walls_of_space_iter<'a>(&'a self, spaceid: &'a str) -> impl Iterator<Item = &'a Wall> {
        self.walls.iter().filter(move |w| {
            w.space == spaceid
                || (if let Some(ref spc) = w.nextto {
                    spc == spaceid
                } else {
                    false
                })
        })
    }

    /// Iterador de los cerramientos de la envolvente térmica en contacto con el aire o el terreno
    /// Se excluyen los opacos sin espacio definido
    pub fn walls_of_envelope_iter(&self) -> impl Iterator<Item = &Wall> {
        self.walls
            .iter()
            .filter(|w| [BoundaryType::EXTERIOR, BoundaryType::GROUND].contains(&w.bounds))
            .filter(move |w| {
                // Si el espacio no está definido se considera que no pertenece a la envolvente
                self.space_by_id(&w.space)
                    .map(|s| s.inside_tenv)
                    .unwrap_or(false)
            })
    }

    /// Iterador de los huecos de la envolvente térmica en contacto con el aire exterior
    /// Se excluyen los huecos sin espacio definido
    pub fn windows_of_envelope_iter(&self) -> impl Iterator<Item = &Window> {
        self.walls
            .iter()
            .filter(|w| w.bounds == BoundaryType::EXTERIOR)
            .filter(move |w| {
                self.space_by_id(&w.space)
                    .map(|s| s.inside_tenv)
                    .unwrap_or(false)
            })
            .flat_map(move |wall| self.windows.iter().filter(move |w| w.wall == wall.id))
    }

    /// Calcula la superficie útil de los espacios habitables de la envolvente térmica [m²]
    pub fn a_ref(&self) -> f32 {
        let a_util: f32 = self
            .spaces
            .iter()
            .filter_map(|s| {
                if s.inside_tenv && s.space_type != SpaceType::UNINHABITED {
                    Some(s.area * s.multiplier)
                } else {
                    None
                }
            })
            .sum();
        fround2(a_util)
    }

    /// Calcula el volumen bruto de los espacios de la envolvente [m³]
    /// Computa el volumen de todos los espacios (habitables o no) de la envolvente
    pub fn vol_env_gross(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .iter()
            .filter_map(|s| {
                if s.inside_tenv {
                    Some(s.area * s.height * s.multiplier)
                } else {
                    None
                }
            })
            .sum();
        fround2(v_env)
    }
    /// Calcula el volumen neto de los espacios de la envolvente [m³]
    /// Computa el volumen de todos los espacios (habitables o no) de la envolvente y
    /// descuenta los volúmenes de forjados y cubiertas
    pub fn vol_env_net(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .iter()
            .filter_map(|s| {
                if s.inside_tenv {
                    Some(s.area * (s.height - self.top_wall_thickness_of_space(&s.id)) * s.multiplier)
                } else {
                    None
                }
            })
            .sum();
        fround2(v_env)
    }
    /// Calcula el volumen neto de los espacios habitables de la envolvente [m³]
    /// Computa el volumen de todos los espacios (solo habitables) de la envolvente y
    /// descuenta los volúmenes de forjados y cubiertas
    pub fn vol_env_inh_net(&self) -> f32 {
        let v_env: f32 = self
            .spaces
            .iter()
            .filter_map(|s| {
                if s.inside_tenv && s.space_type != SpaceType::UNINHABITED {
                    Some(s.area * (s.height - self.top_wall_thickness_of_space(&s.id)) * s.multiplier)
                } else {
                    None
                }
            })
            .sum();
        fround2(v_env)
    }

    /// Calcula la compacidad de la envolvente térmica del edificio V/A (m³/m²)
    /// De acuerdo con la definición del DB-HE comprende el volumen interior de la envolvente térmica (V)
    /// y la superficie de muros y huecos con intercambio térmico con el aire exterior o el terreno (A)
    /// Esta superficie tiene en cuenta los multiplicadores de espacios
    /// Se excluyen los huecos sin muro definido y los muros sin espacio definido
    /// Para area expuesta => compacidad = 0.0
    pub fn compacity(&self) -> f32 {
        let vol: f32 = self.vol_env_gross();
        let area: f32 = self
            .walls_of_envelope_iter()
            .map(|w| {
                let multiplier = self
                    .space_of_wall(&w)
                    .map(|s| s.multiplier)
                    .unwrap_or(1.0);
                let win_area: f32 = self.wincons_of_window_iter(&w.id).map(|win| win.area).sum();
                (w.area + win_area) * multiplier
            })
            .sum();
        let compac = if area == 0.0 { 0.0 } else { vol / area };
        info!("V/A={:.2} m³/m², V={:.2} m³, A={:.2} m²", compac, vol, area);
        compac
    }

    /// Grosor del forjado superior de un espacio
    /// TODO: la altura neta debería calcularse promediando los grosores de todos los muros que cierren el espacio,
    /// TODO: estos podrían ser más de uno pero este cálculo ahora se hace con el primero que se localiza
    pub fn top_wall_thickness_of_space(&self, spaceid: &str) -> f32 {
        self.top_wall_of_space(&spaceid)
            .map(|w| self.wall_thickness(&w.id))
            .unwrap_or(0.0)
    }

    /// Elemento opaco de techo de un espacio
    fn top_wall_of_space<'a>(&'a self, spaceid: &'a str) -> Option<&'a Wall> {
        self.walls.iter().find(move |w| {
            match w.geometry.tilt.into() {
                // Muros exteriores o cubiertas sobre el espacio
                Tilt::TOP => w.space == spaceid,
                // Es un cerramiento interior sobre este espacio
                Tilt::BOTTOM => w.nextto.as_ref().map(|s| s == spaceid).unwrap_or(false),
                _ => false,
            }
        })
    }
}
