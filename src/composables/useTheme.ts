import type { GlobalThemeOverrides } from 'naive-ui';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { usePreferredDark } from '@vueuse/core';
import { darkTheme } from 'naive-ui';
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';
import { useAppConfig } from './useAppConfig';

export function useTheme() {
  const { themeModel, themeColor, font, fontSize } = useAppConfig();

  const prefersDark = usePreferredDark();

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

  const HTML = document.documentElement;

  function getThemeName() {
    return isDark.value ? 'dark' : 'light';
  }

  function setThemeClass() {
    HTML.className = getThemeName();
  }

  async function setThemeModel(newModel: ThemeModel) {
    themeModel.value = newModel;
    let newTheme: string;
    if (themeModel.value === 'system') {
      newTheme = prefersDark.value ? 'dark' : 'light';
    } else {
      newTheme = isDark.value ? 'light' : 'dark';
    }
    const currentWindow = await getCurrentWindow();
    EventBus.emit(AppEvent.CHANGE_THEME, currentWindow.label);
    return newTheme;
  }

  function setHTMLThemeClass(val?: string, pointerEvent?: PointerEvent) {
    val ??= getThemeName();

    // @ts-ignore - ViewTransition API 非标准，仅 Chrome/Edge 支持
    if (document?.startViewTransition) {
      // @ts-ignore
      const transition = document.startViewTransition(() => (HTML.className = val));

      transition.ready.then(() => {
        const pseudo = isDark.value ? '::view-transition-new(root)' : '::view-transition-old(root)';

        // ===== 圆形扩散 =====
        const x = pointerEvent?.clientX ?? innerWidth / 2;
        const y = pointerEvent?.clientY ?? innerHeight / 2;
        const endRadius = Math.hypot(Math.max(x, innerWidth - x), Math.max(y, innerHeight - y));
        const clipPath = [`circle(0px at ${x}px ${y}px)`, `circle(${endRadius}px at ${x}px ${y}px)`];
        document.documentElement.animate(
          { clipPath: isDark.value ? clipPath : [...clipPath].reverse() },
          { duration: 400, easing: 'ease-in', fill: 'forwards', pseudoElement: pseudo },
        );

        // ===== 圆形扩散 + 模糊 =====
        // document.documentElement.animate(
        //   {
        //     clipPath: isDark.value ? clipPath : [...clipPath].reverse(),
        //     filter: ['blur(12px)', 'blur(0px)'],
        //   },
        //   {
        //     duration: 400,
        //     easing: 'ease-in',
        //     fill: 'forwards',
        //     pseudoElement: pseudo,
        //   },
        // );

        // // ===== 菱形扩散 =====
        // const diamond = [
        //   `polygon(50% 50%, 50% 50%, 50% 50%, 50% 50%)`,
        //   `polygon(50% -50%, 150% 50%, 50% 150%, -50% 50%)`,
        // ];
        // document.documentElement.animate(
        //   { clipPath: isDark.value ? diamond : [...diamond].reverse() },
        //   { duration: 400, easing: 'ease-in', fill: 'forwards', pseudoElement: pseudo },
        // );

        // ===== 左侧滑入 =====
        // document.documentElement.animate(
        //   { transform: isDark.value ? ['translateX(-100%)', 'translateX(0)'] : ['translateX(0)', 'translateX(-100%)'] },
        //   { duration: 400, easing: 'ease-in-out', fill: 'forwards', pseudoElement: pseudo },
        // );

        // ===== 右侧滑入 =====
        // document.documentElement.animate(
        //   { transform: isDark.value ? ['translateX(100%)', 'translateX(0)'] : ['translateX(0)', 'translateX(100%)'] },
        //   { duration: 400, easing: 'ease-in-out', fill: 'forwards', pseudoElement: pseudo },
        // );

        // // ===== 顶部滑入 =====
        // document.documentElement.animate(
        //   { transform: isDark.value ? ['translateY(-100%)', 'translateY(0)'] : ['translateY(0)', 'translateY(-100%)'] },
        //   { duration: 400, easing: 'ease-in-out', fill: 'forwards', pseudoElement: pseudo },
        // );

        // // ===== 底部滑入 =====
        // document.documentElement.animate(
        //   { transform: isDark.value ? ['translateY(100%)', 'translateY(0)'] : ['translateY(0)', 'translateY(100%)'] },
        //   { duration: 400, easing: 'ease-in-out', fill: 'forwards', pseudoElement: pseudo },
        // );

        // // ===== 淡入淡出 =====
        // document.documentElement.animate(
        //   { opacity: isDark.value ? ['0', '1'] : ['1', '0'] },
        //   { duration: 400, easing: 'ease-in-out', fill: 'forwards', pseudoElement: pseudo },
        // );

        // ===== 缩放 =====
        // document.documentElement.animate(
        //   { transform: isDark.value ? ['scale(0)', 'scale(1)'] : ['scale(1)', 'scale(0)'] },
        //   { duration: 400, easing: 'ease-in', fill: 'forwards', pseudoElement: pseudo },
        // );

        // ===== 水平百叶窗 =====
        // const blinds = {
        //   clipPath: [`inset(0 100% 0 0)`, `inset(0 0 0 0)`],
        // };
        // document.documentElement.animate(isDark.value ? blinds : { clipPath: blinds.clipPath.slice().reverse() }, {
        //   duration: 400,
        //   easing: 'ease-in-out',
        //   fill: 'forwards',
        //   pseudoElement: pseudo,
        // });

        // // ===== 垂直分割 =====
        // const verticalSplit = [
        //   `inset(0 50% 0 50%)`,
        //   `inset(0 0 0 0)`,
        // ];
        // document.documentElement.animate(
        //   { clipPath: isDark.value ? verticalSplit : [...verticalSplit].reverse() },
        //   { duration: 400, easing: 'ease-in-out', fill: 'forwards', pseudoElement: pseudo },
        // );

        // // ===== 翻页 =====
        // document.documentElement.animate(
        //   {
        //     transform: isDark.value
        //       ? ['perspective(1200px) rotateY(0deg)', 'perspective(1200px) rotateY(-90deg)']
        //       : ['perspective(1200px) rotateY(0deg)', 'perspective(1200px) rotateY(90deg)'],
        //     opacity: isDark.value ? ['1', '0.3'] : ['1', '0.3'],
        //     transformOrigin: isDark.value ? 'left center' : 'right center',
        //   },
        //   { duration: 400, easing: 'ease-in', fill: 'forwards', pseudoElement: '::view-transition-old(root)' },
        // );
        // document.documentElement.animate(
        //   {
        //     transform: isDark.value
        //       ? ['perspective(1200px) rotateY(90deg)', 'perspective(1200px) rotateY(0deg)']
        //       : ['perspective(1200px) rotateY(-90deg)', 'perspective(1200px) rotateY(0deg)'],
        //     opacity: isDark.value ? ['0.3', '1'] : ['0.3', '1'],
        //     transformOrigin: isDark.value ? 'right center' : 'left center',
        //   },
        //   { duration: 400, easing: 'ease-out', fill: 'forwards', pseudoElement: '::view-transition-new(root)', delay: 200 },
        // );
      });
    } else {
      HTML.className = val;
    }
  }

  const DEFAULT_FONT = 'Inter, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "Segoe UI", sans-serif';

  function setFontFamily() {
    HTML.style.setProperty('--app-font-family', `"${font.value || DEFAULT_FONT}", sans-serif`);
  }

  function setFontSize() {
    HTML.style.setProperty('--app-font-size', `${fontSize.value || 14}px`);
  }

  return {
    font,
    fontSize,
    themeModel,
    prefersDark,
    isDark,
    naiveTheme,
    themeOverrides,
    setThemeModel,
    setThemeClass,
    setHTMLThemeClass,
    setFontFamily,
    setFontSize,
  };
}
