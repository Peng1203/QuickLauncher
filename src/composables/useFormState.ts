import { ref } from 'vue';

export function useFormState<T extends Record<string, any>>(origin: T) {
  const form = ref<T>(JSON.parse(JSON.stringify(origin)));

  const _initForm = JSON.parse(JSON.stringify(origin));

  const initForm = () => (form.value = JSON.parse(JSON.stringify(_initForm)));

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
