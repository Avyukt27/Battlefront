<script setup lang="ts">
import { useGameStore } from '@/stores/game';

const store = useGameStore();
</script>

<template>
  <div class="flex gap-4">
    <button @click="store.fetchState"
      class="px-6 py-2 bg-slate-800 hover:bg-slate-700 rounded-md border border-slate-700 transition-colors">
      Sync
    </button>
    <button
      class="px-6 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-md shadow-lg shadow-indigo-500/20 transition-transform active:scale-95"
      @click="store.rollDice" :disabled="store.isRolling">
      <span v-if="store.isRolling">Rolling...</span>
      <span v-else>Roll Dice</span>
    </button>
  </div>
  <div class="text-xl text-indigo-400">Current Roll: {{ store.gameState?.last_roll }}</div>
</template>
