import { ref, type Ref } from 'vue';

// prettier-ignore
type UseLoadingReturn<T extends string = ''> =
  T extends '' ?
  {
    loading: Ref<boolean>;
    startLoading: () => void;
    stopLoading: () => void
  } :
  { [K in `${T & string}Loading`]: Ref<boolean> } &
  { [K in `start${Capitalize<T & string>}Loading`]: () => void } &
  { [K in `stop${Capitalize<T & string>}Loading`]: () => void }

export function useLoading<T extends string = ''>(key?: T): UseLoadingReturn<T> {
  const loading = ref(false);

  const startLoading = () => {
    loading.value = true;
    if (key) console.log(`[${key}] loading start`);
  };

  const stopLoading = () => {
    loading.value = false;
    if (key) console.log(`[${key}] loading end`);
  };

  const upperKey = key ? key.charAt(0).toUpperCase() + key.slice(1) : '';

  return {
    [`${key ? key + 'Loading' : 'loading'}`]: loading,
    [`${key ? 'start' + upperKey + 'Loading' : 'startLoading'}`]: startLoading,
    [`${key ? 'stop' + upperKey + 'Loading' : 'stopLoading'}`]: stopLoading,
  } as UseLoadingReturn<T>;
}

type A = 'ac';

// in 后面可以使用联合类型
type Test = { [K in A]: string };
