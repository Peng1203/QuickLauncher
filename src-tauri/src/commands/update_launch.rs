use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{
    common::utils::get_pinyin_variants, entity::launch_items, models::launch_item::LaunchItemDto,
};

#[tauri::command]
pub async fn update_launch(
    item: LaunchItemDto,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    // ✅ 1. 先查是否存在
    let model = launch_items::Entity::find_by_id(item.id)
        .one(db.inner())
        .await
        .map_err(|e| format!("查询失败: {}", e))?;

    let model = match model {
        Some(m) => m,
        None => return Err("No launch item found with the specified ID".to_string()),
    };

    // ✅ 2. 拼音生成（业务逻辑）
    let (pinyin_full, pinyin_abbr) = get_pinyin_variants(&item.name);

    // ✅ 3. 转 ActiveModel
    let mut active: launch_items::ActiveModel = model.into();

    // ✅ 4. 赋值（等价 SQL SET）
    active.name = Set(item.name);
    active.path = Set(item.path);
    active.r#type = Set(item.r#type);
    active.icon = Set(item.icon);

    active.pinyin_full = Set(Some(pinyin_full));
    active.pinyin_abbr = Set(Some(pinyin_abbr));

    active.extension = Set(item.extension);
    active.hotkey = Set(item.hotkey);
    active.hotkey_global = Set(item.hotkey_global);
    active.keywords = Set(item.keywords);
    active.start_dir = Set(item.start_dir);
    active.remarks = Set(item.remarks);
    active.args = Set(item.args);
    active.run_as_admin = Set(item.run_as_admin);
    active.order_index = Set(item.order_index);
    active.enabled = Set(item.enabled);
    active.category_id = Set(item.category_id);

    // ✅ 5. 执行更新
    active
        .update(db.inner())
        .await
        .map_err(|e| format!("更新失败: {}", e))?;

    Ok(())
}
