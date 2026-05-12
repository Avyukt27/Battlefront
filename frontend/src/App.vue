<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';
import { useGameStore } from './stores/game';
import GameBoard from './components/game/GameBoard.vue';
import GameControls from './components/game/GameControls.vue';
import LobbyView from './components/lobby/LobbyView.vue';
import GameCard from './components/game/GameCard.vue';

const store = useGameStore();
let pollInterval: number;

const myPlayer = computed(() => store.gameState?.players.find((p) => p.id === store.myPlayerId));

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
      <p class="mt-4 text-slate-500 italic">Connecting to game...</p>
    </section>

    <section v-else class="flex flex-col items-center gap-6"></section>

    <section v-if="store.gameState" class="flex flex-col items-center gap-6">
      <div class="text-center">
        <p class="text-slate-500 text-xs uppercase tracking-widest">Room</p>
        <p class="text-xl font-mono text-indigo-400">{{ store.gameId }}</p>
      </div>

      <div class="flex flex-col lg:flex-row gap-8 items-start justify-center">
        <div class="shrink-0 shadow-2xl rounded-xl overflow-hidden border border-slate-800">
          <GameBoard />
        </div>

        <aside class="flex flex-col gap-6">
          <div class="bg-slate-900/50 p-4 rounded-2xl border border-slate-800 backdrop-blur-sm">
            <h2 class="text-white font-bold text-lg mb-4 flex items-center gap-2">
              <span class="w-2 h-2 bg-indigo-500 rounded-full"></span>
              Your Hand
            </h2>

            <div class="flex flex-col gap-4 items-center">
              <GameCard v-for="card in myPlayer?.cards" :key="card.id" :card="card.name" />
            </div>
          </div>
        </aside>
      </div>
      <div class="border-t border-slate-800 pt-6 min-w-full">
        <GameControls />
      </div>
    </section>
  </div>
</template>
