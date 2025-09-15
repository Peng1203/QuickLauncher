#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigState {
    pub silent_start: bool,
    // 开机自启
    pub auto_start: bool,
    // 窗口置顶
    pub on_top: bool,
    // 窗口居中
    pub center: bool,
    // 搜索窗口最大高度
    pub search_window_max_height: i32,
    // 搜索窗口固定宽度
    pub search_window_width: i32,
    // 搜索框高度
    pub search_window_input: i32,
    // 搜索结果高度
    pub search_result_item_height: i32,
    // 全局快捷键
    #[serde(default)]
    pub search_global_shortcut_key: String,

    // 是否启用代理
    pub proxy: bool,
    // 代理主机
    pub proxy_host: String,
    // 代理用户名
    pub proxy_username: String,
    // 代理密码
    pub proxy_password: String,

    pub main_window_position_x: i32,
    pub main_window_position_y: i32,
}
