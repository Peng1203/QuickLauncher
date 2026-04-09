import type { Directive } from 'vue';

export type TransparentDirective = Directive<HTMLElement, boolean>;

declare module 'vue' {
  export interface ComponentCustomProperties {
    // 使用 v 作为前缀 (v-transparent)
    vTransparent: TransparentDirective;
  }
}

function update(el: HTMLElement, hidden: boolean) {
  el.style.opacity = hidden ? '0' : '1';
  el.style.pointerEvents = hidden ? 'none' : '';
  el.style.transition = 'opacity 0.2s ease';
}

export default {
  // 在绑定元素的 attribute 前
  // 或事件监听器应用前调用
  created(el, binding, vnode) {
    const { value } = binding;
    update(el, value);
    // 下面会介绍各个参数的细节
  },
  // 在元素被插入到 DOM 前调用
  beforeMount(el, binding, vnode) {},
  // 在绑定元素的父组件
  // 及他自己的所有子节点都挂载完成后调用
  mounted: (el, binding) => {
    // el.style.backgroundColor = binding.value;
  },
  // 绑定元素的父组件更新前调用
  beforeUpdate(el, binding, vnode, prevVnode) {},
  // 在绑定元素的父组件
  // 及他自己的所有子节点都更新后调用
  updated(el, binding, vnode, prevVnode) {
    update(el, binding.value);
  },
  // 绑定元素的父组件卸载前调用
  beforeUnmount(el, binding, vnode) {},
  // 绑定元素的父组件卸载后调用
  unmounted(el, binding, vnode) {},
} satisfies TransparentDirective;
