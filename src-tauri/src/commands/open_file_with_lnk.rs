use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
    COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Shell::{
    FileOpenDialog,
    IFileOpenDialog,
    IShellItem,
    FOS_NODEREFERENCELINKS, // 关键：不解析快捷方式
    SIGDN_FILESYSPATH,
};

#[tauri::command]
#[cfg(target_os = "windows")]
pub fn open_file_with_lnk() -> Result<String, String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .map_err(|e| e.to_string())?;

        let dialog: IFileOpenDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER)
            .map_err(|e| e.to_string())?;

        // 设置 FOS_NODEREFERENCELINKS 阻止自动解析 .lnk
        let mut options = dialog.GetOptions().map_err(|e| e.to_string())?;
        options |= FOS_NODEREFERENCELINKS;
        dialog.SetOptions(options).map_err(|e| e.to_string())?;

        // 显示对话框
        dialog.Show(None).map_err(|e| e.to_string())?;

        // 获取选中的文件路径（此时是 .lnk 原始路径）
        let item: IShellItem = dialog.GetResult().map_err(|e| e.to_string())?;
        let path = item
            .GetDisplayName(SIGDN_FILESYSPATH)
            .map_err(|e| e.to_string())?;

        CoUninitialize();

        Ok(path.to_string().map_err(|e| e.to_string())?)
    }
}
