<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useGameStore } from './stores/game';

const store = useGameStore();
let pollInterval: number;

onMounted(() => {
  store.fetchState();

  pollInterval = setInterval(() => {
    store.fetchState();
  }, 1500);
});

onUnmounted(() => {
  clearInterval(pollInterval);
});
</script>

<template>
  <h1>Hello World!</h1>
</template>
