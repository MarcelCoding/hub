<template>
  <div class="container">
    <main>
      <header v-if="meta && meta.header">
        <h1>{{ meta.header.name }}</h1>
        <p>{{ meta.header.description }}</p>
      </header>

      <section v-for="group in groups" :key="group.name">
        <h3>{{ group.name }}</h3>
        <table>
          <tr v-for="service in group.services" :key="service.id">
            <td><a :href="service.url">{{ prettyUrl(service.url) }}</a></td>
            <td>- {{ service.id }}</td>
            <td class="description">{{ service.description }}</td>
          </tr>
        </table>
      </section>

      <p v-if="meta">You want another application? Feel free to write an email to <a
        :href="'mailto:' + meta.contact">{{ meta.contact }}</a>!
      </p>
    </main>

    <HubFooter v-if="meta" :url="meta.url" :contact="meta.contact"/>
  </div>
</template>

<script setup lang="ts">
import HubFooter from './components/HubFooter.vue';
import {prettyUrl, useFetch} from "./fetch";
import {Group, Meta} from "./domain";

const {data: meta, error: _} = useFetch<Meta>("http://localhost:3030/meta");
const {data: groups, error: _2} = useFetch<Group[]>("http://localhost:3030/services");
</script>

<style scoped lang="scss">
.description {
  font-style: italic;
}

main {
  margin-bottom: auto;
}

.container {
  height: 100vh;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  align-content: space-between;
}
</style>
