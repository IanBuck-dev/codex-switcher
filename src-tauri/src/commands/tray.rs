use tauri::{AppHandle, menu::{MenuBuilder, MenuItemBuilder}};

#[tauri::command]
pub fn update_tray_info(
    app: AppHandle,
    active_name: Option<String>,
    primary_left: Option<f64>,
    secondary_left: Option<f64>
) {
    if let Some(tray) = app.tray_by_id("main") {
        let p_str = primary_left.map(|v| format!("{:.0}%", v)).unwrap_or_else(|| "-".to_string());
        let s_str = secondary_left.map(|v| format!("{:.0}%", v)).unwrap_or_else(|| "-".to_string());
        
        // Update the menu bar title (text next to icon)
        if let Some(ref name) = active_name {
            let tray_title = format!("{} • {} / {}", name, p_str, s_str);
            let _ = tray.set_title(Some(tray_title));
        } else {
            let _ = tray.set_title(None::<String>);
        }

        let title_item = MenuItemBuilder::with_id("title", "Codex Switcher").enabled(false).build(&app).unwrap();
        let mut builder = MenuBuilder::new(&app).item(&title_item).separator();
        
        if let Some(name) = active_name {
            let name_item = MenuItemBuilder::with_id("active", format!("Active: {}", name)).enabled(false).build(&app).unwrap();
            builder = builder.item(&name_item);
        }
        
        if primary_left.is_some() || secondary_left.is_some() {
            let usage_p = MenuItemBuilder::with_id("usage_p", format!("5h Limit Left: {}", p_str)).enabled(false).build(&app).unwrap();
            let usage_s = MenuItemBuilder::with_id("usage_s", format!("Weekly Limit Left: {}", s_str)).enabled(false).build(&app).unwrap();
            builder = builder.item(&usage_p).item(&usage_s);
        }
        
        let show_item = MenuItemBuilder::with_id("show", "Show App").build(&app).unwrap();
        let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(&app).unwrap();
        
        let menu = builder
            .separator()
            .item(&show_item)
            .item(&quit_item)
            .build()
            .unwrap();
            
        let _ = tray.set_menu(Some(menu));
    }
}
