use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn get_storage_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create dir: {}", e))?;
    Ok(dir.join("secure_storage.json"))
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Storage {
    license_key: Option<String>,
    instance_id: Option<String>,
    selected_pluely_model: Option<String>,
}

type StorageSecure = Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageItem {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageResult {
    license_key: Option<String>,
    instance_id: Option<String>,
    selected_pluely_model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ActivationResponse {
    activated: bool,
    error: Option<String>,
    license_key: Option<String>,
    is_dev_license: bool,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateResponse {
    is_active: bool,
    is_dev_license: bool,
    last_validated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutResponse {
    success: Option<bool>,
    checkout_url: Option<String>,
    error: Option<String>,
}

#[tauri::command]
pub async fn secure_storage_save(app: AppHandle, items: Vec<StorageItem>) -> Result<(), String> {
    let path = get_storage_path(&app)?;
    let mut storage: StorageSecure = if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default()
    } else {
        StorageSecure::default()
    };

    for item in items {
        match item.key.as_str() {
            "pluely_license_key" => storage.license_key = Some(item.value),
            "pluely_instance_id" => storage.instance_id = Some(item.value),
            "selected_pluely_model" => storage.selected_pluely_model = Some(item.value),
            _ => return Err(format!("Invalid storage key: {}", item.key)),
        }
    }

    fs::write(&path, serde_json::to_string(&storage)?).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn secure_storage_get(app: AppHandle) -> Result<StorageResult, String> {
    let path = get_storage_path(&app)?;
    if !path.exists() {
        return Ok(StorageResult {
            license_key: None,
            instance_id: None,
            selected_pluely_model: None,
        });
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let storage: StorageSecure = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(StorageResult {
        license_key: storage.license_key,
        instance_id: storage.instance_id,
        selected_pluely_model: storage.selected_pluely_model,
    })
}

#[tauri::command]
pub async fn secure_storage_remove(app: AppHandle, keys: Vec<String>) -> Result<(), String> {
    let path = get_storage_path(&app)?;
    if !path.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut storage: StorageSecure = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    for key in keys {
        match key.as_str() {
            "pluely_license_key" => storage.license_key = None,
            "pluely_instance_id" => storage.instance_id = None,
            "selected_pluely_model" => storage.selected_pluely_model = None,
            _ => return Err(format!("Invalid storage key: {}", key)),
        }
    }
    fs::write(&path, serde_json::to_string(&storage)?).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn activate_license_api(_app: AppHandle, _license_key: String) -> Result<ValidationResponse, String> {
    Ok(ValidationResponse { active: true, message: "Enabled".into() })
}

#[tauri::command]
pub async fn deactivate_license_api(_app: AppHandle) -> Result<ValidationResponse, String> {
    Ok(ValidationResponse { active: true, message: "Enabled".into() })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResponse {
    active: bool,
    message: String,
}

#[tauri::command]
pub async fn validate_license_api(_app: AppHandle) -> Result<ValidateResponse, String> {
    Ok(ValidateResponse {
        is_active: true,
        is_dev_license: false,
        last_validated_at: None,
    })
}

#[tauri::command]
pub fn mask_license_key_cmd(license_key: String) -> String {
    if license_key.len() <= 4 {
        return "*".repeat(license_key.len());
    }
    let first = &license_key[..4];
    let stars = "*".repeat(license_key.len() - 4);
    format!("{}{}", first, stars)
}

#[tauri::command]
pub async fn get_checkout_url() -> Result<CheckoutResponse, String> {
    Ok(CheckoutResponse {
        success: None,
        checkout_url: None,
        error: None,
    })
}