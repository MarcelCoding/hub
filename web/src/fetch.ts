import {isRef, ref, unref, watchEffect} from 'vue'

export function useFetch<T>(url: string) {
  const data = ref<T | null>(null)
  const error = ref<T | null>(null)

  function doFetch() {
    // reset state before fetching..
    data.value = null
    error.value = null
    // unref() unwraps potential refs
    fetch(unref(url))
      .then((res) => res.json())
      .then((json) => (data.value = json))
      .catch((err) => (error.value = err))
  }

  if (isRef(url)) {
    // setup reactive re-fetch if input URL is a ref
    watchEffect(doFetch)
  }
  else {
    // otherwise, just fetch once
    // and avoid the overhead of a watcher
    doFetch()
  }

  return {data, error}
}

export function prettyUrl(url: string) {
  if (url.startsWith("https://")) {
    url = url.substring(8);
  }
  if (url.endsWith('/')) {
    url = url.substring(0, url.length - 1);
  }
  return url;
}
