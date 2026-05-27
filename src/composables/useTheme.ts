import type { GlobalThemeOverrides } from 'naive-ui';
import { darkTheme } from 'naive-ui';
import { useAppConfig } from './useAppConfig';

export function useTheme() {
  const { themeModel, themeColor, font, fontSize } = useAppConfig();

  const prefersDark = ref(window.matchMedia('(prefers-color-scheme: dark)').matches);

  const isDark = computed(() => {
    if (themeModel.value === 'dark') return true;
    if (themeModel.value === 'light') return false;
    return prefersDark.value;
  });

  const naiveTheme = computed(() => (isDark.value ? darkTheme : undefined));

  const themeOverrides = computed<GlobalThemeOverrides>(() => ({
    common: {
      primaryColor: themeColor.value,
      primaryColorHover: themeColor.value,
    },
  }));

  const HTML = document.documentElement as HTMLElement;
  // 获取最新的主题 light/dark
  function getChangeToModel() {
    return isDark.value ? 'dark' : 'light';
  }
  function initThemeClass() {
    HTML.className = getChangeToModel();
  }

  function setThemeModel() {
    const newTheme = isDark.value ? 'light' : 'dark';
    themeModel.value = newTheme;
    return newTheme;
  }

  function setHTMLThemeClass(val?: string, pointerEvent?: PointerEvent) {
    !val && (val = getChangeToModel());

    /**
     * ViewTransition API 只有 Chrome 和 Edge 以及部分浏览器才支持 并不是 标注的API
     */

    // @ts-ignore
    if (document?.startViewTransition) {
      // 没有 event 时默认从窗口中心开始扩散
      const x = pointerEvent?.clientX ?? innerWidth / 2;
      const y = pointerEvent?.clientY ?? innerHeight / 2;

      const endRadius = Math.hypot(Math.max(x, innerWidth - x), Math.max(y, innerHeight - y));
      // @ts-ignore
      const transition = document.startViewTransition(() => (HTML.className = val));

      transition.ready.then(() => {
        const clipPath = [`circle(0px at ${x}px ${y}px)`, `circle(${endRadius}px at ${x}px ${y}px)`];
        document.documentElement.animate(
          {
            clipPath: isDark.value ? clipPath : [...clipPath].reverse(),
          },
          {
            duration: 400,
            easing: 'ease-in',
            pseudoElement: isDark.value ? '::view-transition-new(root)' : '::view-transition-old(root)',
          },
        );
      });
    } else {
      HTML.className = val;
    }
  }

  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  function onOsThemeChange(e: MediaQueryListEvent) {
    handleOnOSThemeChange(e.matches);
  }

  function handleOnOSThemeChange(isDark: boolean) {
    if (themeModel.value !== 'system') return;
    // 当系统切到黑色模式是 e.matches 是true 白色默认时为false
    prefersDark.value = isDark;
    const newTheme = isDark ? 'dark' : 'light';
    setHTMLThemeClass(newTheme);
  }

  function setFontFamily() {
    if (font.value) {
      document.documentElement.style.setProperty('--app-font-family', `"${font.value}", sans-serif`);
    } else {
      // 设置默认字体
      const defaultFont = `Inter, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "Segoe UI", sans-serif`;
      document.documentElement.style.setProperty('--app-font-family', defaultFont);
    }
  }

  function setFontSize() {
    if (fontSize.value) {
      document.documentElement.style.setProperty('--app-font-size', `${fontSize.value}px`);
    } else {
      document.documentElement.style.setProperty('--app-font-size', `14px`);
    }
  }

  return {
    font,
    fontSize,
    mediaQuery,
    themeModel,
    prefersDark,
    isDark,
    naiveTheme,
    themeOverrides,
    setThemeModel,
    initThemeClass,
    onOsThemeChange,
    setHTMLThemeClass,
    handleOnOSThemeChange,
    setFontFamily,
    setFontSize,
  };
}
