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

const getPlayerAt = (x: number, y: number) => {
  return store.gameState?.players.find((p) => p.x === x && p.y === y);
};
</script>

<template>
  <div class="min-h-screen bg-slate-950 text-slate-100 font-sans">
    <div class="p-8">
      <h1 class="text-4xl font-bold mb-4 text-center">Battlefront</h1>
    </div>

    <section v-if="store.gameState" class="flex flex-col items-center gap-6">
      <div class="grid grid-cols-8 gap-1 bg-slate-900 p-1 border border-slate-800">
        <template v-for="y in 8" :key="`row-${y}`">
          <div
            v-for="x in 8"
            :key="`cell-${x}-${y}`"
            @click="store.makeMove(1, x - 1, y - 1)"
            class="relative w-12 h-12 sm:w-16 sm:h-16 bg-slate-800/40 border border-slate-700/20 hover:bg-slate-700 transition-all cursor-pointer flex items-center justify-center"
          >
            <div
              v-if="getPlayerAt(x - 1, y - 1)"
              class="w-4/5 h-4/5 rounded-full shadow-2xl transition-all duration-500 transform scale-90"
              :class="{
                'bg-red-600 border-red-400': getPlayerAt(x - 1, y - 1)?.colour === 'Red',
                'bg-blue-600 border-blue-400': getPlayerAt(x - 1, y - 1)?.colour === 'Blue',
              }"
            >
              <div
                v-if="store.gameState?.current_turn === getPlayerAt(x - 1, y - 1)?.colour"
                class="absolute inset-0 rounded-full animate-ping bg-white/20"
              ></div>
            </div>
          </div>
        </template>
      </div>
      <div class="flex gap-4">
        <button
          @click="store.fetchState"
          class="px-6 py-2 bg-slate-800 hover:bg-slate-700 rounded-md border border-slate-700 transition-colors"
        >
          Sync
        </button>
        <button
          class="px-6 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-md shadow-lg shadow-indigo-500/20 transition-transform active:scale-95"
          @click="store.rollDice"
          :disabled="store.isRolling"
        >
          <span v-if="store.isRolling">Rolling...</span>
          <span v-else>Roll Dice</span>
        </button>
      </div>

      <div class="text-xl text-indigo-400">Current Roll: {{ store.gameState?.last_roll }}</div>
    </section>

    <section v-else class="flex flex-col items-center justify-center py-20">
      <div
        class="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin"
      ></div>
      <p class="mt-4 text-slate-500 italic">Waiting for server...</p>
    </section>
  </div>
</template>
