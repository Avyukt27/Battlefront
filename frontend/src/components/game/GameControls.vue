<script setup lang="ts">
import { useGameStore } from '@/stores/game';
import { computed } from 'vue';

const store = useGameStore();

const myPlayer = computed(() => store.gameState?.players.find((p) => p.id === store.myPlayerId));
const isMyTurn = computed(() => store.gameState?.current_turn === myPlayer.value?.colour);
const roll = computed(() => store.gameState?.last_roll);
</script>

<template>
  <div class="flex gap-4">
    <button @click="store.fetchState"
      class="px-6 py-2 bg-slate-800 hover:bg-slate-700 rounded-md border border-slate-700 transition-colors">
      Sync
    </button>
    <button
      class="px-6 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-md shadow-lg shadow-indigo-500/20 transition-transform active:scale-95"
      @click="store.rollDice" :disabled="store.isRolling || roll !== 0 || !isMyTurn">
      <span v-if="store.isRolling && roll === 0">Rolling...</span>
      <span v-else-if="roll !== 0">Waiting for {{ store.gameState?.current_turn }}</span>
      <span v-else>Roll Dice</span>
    </button>
    <button @click="store.leaveGame"
      class="px-6 py-2 bg-red-500 hover:bg-red-400 rounded-md border border-red-400 transition-colors">
      Leave Game
    </button>
  </div>
  <div v-if="roll !== 0 && isMyTurn" class="text-xl text-indigo-400">Roll: {{ roll }}</div>
</template>
