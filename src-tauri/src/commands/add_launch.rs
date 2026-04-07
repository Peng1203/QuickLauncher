use crate::{entity, models::launch_item::NewLaunchItem};
use entity::prelude::LaunchItems;
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::common::utils::get_pinyin_variants;

#[tauri::command]
pub async fn add_launch(
    item: NewLaunchItem,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    dbg!(&item);
    let pinyin_value = get_pinyin_variants(&item.name);
    let pinyin_full = pinyin_value.0;
    let pinyin_abbr = pinyin_value.1;

    let model = entity::launch_items::ActiveModel {
        name: Set(item.name.clone()),
        lnk_name: Set(item.lnk_name.clone()),
        path: Set(item.path.clone()),
        r#type: Set(item.r#type.clone()),
        icon: Set(item.icon),
        pinyin_full: Set(Some(pinyin_full)),
        pinyin_abbr: Set(Some(pinyin_abbr)),
        extension: Set(item.extension.clone()),
        hotkey: Set(item.hotkey.clone()),
        hotkey_global: Set(item.hotkey_global.clone()),
        keywords: Set(item.keywords.clone()),
        start_dir: Set(item.start_dir.clone()),
        remarks: Set(item.remarks.clone()),
        args: Set(item.args.clone()),
        run_as_admin: Set(item.run_as_admin.clone()),
        order_index: Set(item.order_index.clone()),
        enabled: Set(item.enabled),
        category_id: Set(item.category_id),
        subcategory_id: Set(item.subcategory_id),
        ..Default::default()
    };

    LaunchItems::insert(model)
        .exec(&*db)
        .await
        .map_err(|e| format!("插入启动项失败: {}", e))?;

    Ok(())
}
