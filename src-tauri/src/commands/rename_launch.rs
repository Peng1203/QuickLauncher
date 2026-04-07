use crate::{common::utils::get_pinyin_variants, entity};
use entity::launch_items::{ActiveModel, Column, Entity as LaunchItems};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

#[tauri::command]
pub async fn rename_launch(
    id: i32,
    name: String,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<(), String> {
    println!("rename_launch id: {}, name: {}", id, name);

    // 生成拼音
    let (pinyin_full, pinyin_abbr) = get_pinyin_variants(&name);

    // 先查出来
    let model = LaunchItems::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|e| format!("查询失败：{}", e))?
        .ok_or("记录不存在")?;

    // 转 ActiveModel
    let mut active: ActiveModel = model.into();

    // 修改字段
    active.name = Set(name);
    active.pinyin_full = Set(Some(pinyin_full));
    active.pinyin_abbr = Set(Some(pinyin_abbr));

    // 执行更新
    active
        .update(db.inner())
        .await
        .map_err(|e| format!("更新失败：{}", e))?;

    Ok(())
}
