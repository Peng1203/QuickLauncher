// useEsc.ts
import { useEventListener } from "@vueuse/core";

export function useEsc(callback: () => void) {
  const handler = (e: KeyboardEvent) => {
    if (e.key === "Escape") callback();
  };

  useEventListener("keydown", handler);
}
