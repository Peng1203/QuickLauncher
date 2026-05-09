use crate::{common::utils::get_pinyin_variants, entity, AppState};
use entity::launch_items::{ActiveModel, Entity as LaunchItems};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[tauri::command]
pub async fn rename_launch(
    id: i32,
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = { state.db.lock().unwrap().clone() };
    let db = db.ok_or("数据库未连接")?;

    println!("rename_launch id: {}, name: {}", id, name);

    // 生成拼音
    let (pinyin_full, pinyin_abbr) = get_pinyin_variants(&name);

    // 先查出来
    let model = LaunchItems::find_by_id(id)
        .one(&db)
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
        .update(&db)
        .await
        .map_err(|e| format!("更新失败：{}", e))?;

    Ok(())
}
