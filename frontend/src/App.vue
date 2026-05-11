<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useGameStore } from './stores/game';
import GameBoard from './components/game/GameBoard.vue';
import GameControls from './components/game/GameControls.vue';
import LobbyView from './components/lobby/LobbyView.vue';

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
  <div class="min-h-screen bg-slate-950 text-slate-100 font-sans">
    <div class="p-8">
      <h1 class="text-4xl font-bold mb-4 text-center">Battlefront</h1>
    </div>

    <LobbyView v-if="!store.gameId" />

    <section v-else-if="!store.gameState" class="flex flex-col items-center justify-center py-20">
      <div class="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
      <p class="mt-4 text-slate-500 italic">Connecting to theatre...</p>
    </section>

    <section v-else class="flex flex-col items-center gap-6"></section>

    <section v-if="store.gameState" class="flex flex-col items-center gap-6">
      <div class="text-center">
        <p class="text-slate-500 text-xs uppercase tracking-widest">Room</p>
        <p class="text-xl font-mono text-indigo-400">{{ store.gameId }}</p>
      </div>

      <GameBoard />
      <GameControls />
    </section>
  </div>
</template>
