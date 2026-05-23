use crate::models::app_config_state::AppConfigState;
use std::sync::Mutex;
use tauri::State;
use tracing;

#[tracing::instrument(skip(state))]
#[tauri::command]
pub fn set_app_config(config: AppConfigState, state: State<'_, Mutex<AppConfigState>>) {
    let mut state = state.lock().unwrap();
    *state = config;
}
