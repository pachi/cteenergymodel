// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Horarios (SCHEDULE-PD, WEEK-SCHEDULE-PD, DAY-SCHEDULE-PD)

mod schedules;

pub use schedules::{DaySchedule, Schedule, WeekSchedule, YearSchedule};
