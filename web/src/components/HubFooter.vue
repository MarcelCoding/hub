<template>
  <footer>
    <a v-if="props.url" :href="props.url">{{ prettyUrl(props.url) }}</a>
    |
    <span>source: <a href="https://github.com/MarcelCoding/hub">github.com/MarcelCoding/hub</a></span>
    |
    <span>contact: <a :href="'mailto:' + props.contact">{{ props.contact }}</a></span>
  </footer>
</template>

<script setup lang="ts">
import {prettyUrl} from "../fetch";

const props = defineProps<{ url: string, contact: string }>()
</script>

<style scoped lang="scss">
.description {
  font-style: italic;
}

footer {
  display: flex;
  column-gap: 0.5rem;
  justify-content: center;
}
</style>
