use serde_json::Value;

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
    // 搜索框全局快捷键
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
    #[serde(default)]
    pub main_window_global_shortcut_key: String,

    #[serde(default = "default_language")]
    pub language: String,

    #[serde(default)]
    pub enable_search: bool,
    #[serde(default)]
    pub search_lost_focus_hide: bool,
    #[serde(default)]
    pub search_hide_after_open: bool,
    #[serde(default)]
    pub web_search_open_model: i32,
    #[serde(default)]
    pub web_search_source_list: Vec<Value>,

    // Theme
    #[serde(default = "default_appearance_mode")]
    pub appearance_mode: String,
    #[serde(default = "default_theme_color")]
    pub theme_color: String,
    #[serde(default = "default_page_style")]
    pub page_style: String,
    #[serde(default = "default_layout_density")]
    pub layout_density: String,
    #[serde(default = "default_font_size")]
    pub font_size: i32,
    #[serde(default = "default_border_radius")]
    pub border_radius: i32,
    #[serde(default = "default_background_blur")]
    pub background_blur: i32,
    #[serde(default = "default_window_opacity")]
    pub window_opacity: i32,
    #[serde(default = "default_enable_animation")]
    pub enable_animation: bool,
    #[serde(default = "default_animation_speed")]
    pub animation_speed: String,
}

fn default_language() -> String {
    "zh-CN".to_string()
}
fn default_appearance_mode() -> String { "system".into() }
fn default_theme_color() -> String { "#2080f0".into() }
fn default_page_style() -> String { "normal".into() }
fn default_layout_density() -> String { "default".into() }
fn default_font_size() -> i32 { 14 }
fn default_border_radius() -> i32 { 10 }
fn default_background_blur() -> i32 { 50 }
fn default_window_opacity() -> i32 { 100 }
fn default_enable_animation() -> bool { true }
fn default_animation_speed() -> String { "normal".into() }
