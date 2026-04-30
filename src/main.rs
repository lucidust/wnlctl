use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::Serialize;
use win_nightlight_lib::{
    NightlightManager, RegistryBackend,
    nightlight_settings::{NightlightSettings, ScheduleMode},
    nightlight_state::NightlightState,
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print the current Night Light state.
    Status {
        /// Print machine-readable JSON.
        #[arg(long)]
        json: bool,
    },
    /// Turn Night Light on without changing the schedule.
    On {
        /// Print machine-readable JSON after the change.
        #[arg(long)]
        json: bool,
    },
    /// Turn Night Light off without changing the schedule.
    Off {
        /// Print machine-readable JSON after the change.
        #[arg(long)]
        json: bool,
    },
    /// Toggle Night Light without changing the schedule.
    Toggle {
        /// Print machine-readable JSON after the change.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NightLightStatus {
    enabled: bool,
    schedule_mode: String,
    color_temperature_kelvin: u16,
    schedule_start: String,
    schedule_end: String,
    sunset_time: String,
    sunrise_time: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let manager = NightlightManager::new(RegistryBackend);

    match cli.command {
        Command::Status { json } => print_status(&manager, json)?,
        Command::On { json } => {
            set_enabled(&manager, true)?;
            print_status(&manager, json)?;
        }
        Command::Off { json } => {
            set_enabled(&manager, false)?;
            print_status(&manager, json)?;
        }
        Command::Toggle { json } => {
            let state = manager.get_state()?;
            set_enabled(&manager, !state.is_enabled)?;
            print_status(&manager, json)?;
        }
    }

    Ok(())
}

fn set_enabled(manager: &NightlightManager<RegistryBackend>, enabled: bool) -> Result<()> {
    let mut state = manager.get_state()?;
    let changed = if enabled {
        state.enable()
    } else {
        state.disable()
    };

    if changed {
        manager.set_state(&state)?;
    }

    Ok(())
}

fn print_status(manager: &NightlightManager<RegistryBackend>, json: bool) -> Result<()> {
    let settings = manager.get_settings()?;
    let state = manager.get_state()?;
    let status = NightLightStatus::from_parts(&state, &settings);

    if json {
        println!("{}", serde_json::to_string(&status)?);
    } else {
        println!("{}", if status.enabled { "on" } else { "off" });
    }

    Ok(())
}

impl NightLightStatus {
    fn from_parts(state: &NightlightState, settings: &NightlightSettings) -> Self {
        Self {
            enabled: state.is_enabled,
            schedule_mode: schedule_mode_label(settings.schedule_mode).to_string(),
            color_temperature_kelvin: settings.color_temperature,
            schedule_start: settings.start_time.format("%H:%M").to_string(),
            schedule_end: settings.end_time.format("%H:%M").to_string(),
            sunset_time: settings.sunset_time.format("%H:%M").to_string(),
            sunrise_time: settings.sunrise_time.format("%H:%M").to_string(),
        }
    }
}

fn schedule_mode_label(mode: ScheduleMode) -> &'static str {
    match mode {
        ScheduleMode::Off => "off",
        ScheduleMode::SunsetToSunrise => "sunset-to-sunrise",
        ScheduleMode::SetHours => "set-hours",
    }
}
