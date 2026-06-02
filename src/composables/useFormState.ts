import { useCloned } from "@vueuse/core";

export function useFormState<T extends Record<string, any>>(origin: T) {
  const { cloned: form, sync: initForm } = useCloned(origin);

  const setForm = (data: T) => {
    for (const key in form.value) {
      if (data[key]) form.value[key] = data[key];
    }
  };

  return {
    form,
    initForm,
    setForm,
  };
}
