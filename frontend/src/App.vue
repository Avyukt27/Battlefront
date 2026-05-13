<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useGameStore } from './stores/game';
import LobbyView from './components/lobby/LobbyView.vue';
import GameView from './components/game/GameView.vue';

const store = useGameStore();
let pollInterval: number;

onMounted(async () => {
  if (store.gameId) {
    console.log('Reconnecting to game: ', store.gameId);
    await store.fetchState();
  }

  pollInterval = setInterval(() => {
    store.fetchState();
  }, 1500);
});

onUnmounted(() => {
  clearInterval(pollInterval);
});
</script>

<template>
  <div class="h-screen bg-slate-950 text-slate-100 font-sans flex flex-col overflow-hidden">
    <div class="pt-8 pb-1 shrink-0">
      <h1 class="text-4xl font-bold mb-4 text-center">Battlefront</h1>
    </div>

    <LobbyView v-if="!store.gameId" />

    <section v-else-if="!store.gameState" class="flex flex-col items-center justify-center py-20">
      <div class="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
      <p class="mt-4 text-slate-500 italic">Connecting to game...</p>
    </section>

    <GameView v-else />

    <Transition name="fade">
      <div v-if="store.lastError"
        class="fixed bottom-10 left-1/2 -translate-x-1/2 z-200 bg-red-500/90 px-6 py-3 rounded-full shadow-2xl backdrop-blur-md">
        {{ store.lastError }}
      </div>
    </Transition>
  </div>
</template>
