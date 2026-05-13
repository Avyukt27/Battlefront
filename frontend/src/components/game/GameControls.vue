<script setup lang="ts">
import { useGameStore } from '@/stores/game';
import { computed } from 'vue';

const store = useGameStore();

const myPlayer = computed(() => store.gameState?.players.find((p) => p.id === store.myPlayerId));
const isMyTurn = computed(() => store.gameState?.current_turn === myPlayer.value?.colour);
const handFull = computed(() => (myPlayer.value?.cards.length ?? 0) >= 3);
const roll = computed(() => store.gameState?.last_roll);
const playerCount = computed(() => store.gameState?.players.length ?? 0);

const handleDraw = async () => {
  if (!myPlayer.value || handFull.value) return;
  const cardsNeeded = 3 - myPlayer.value.cards.length;
  for (let i = 0; i < cardsNeeded; i++) {
    await store.drawCard();
  }
};
</script>

<template>
  <div class="flex flex-col justify-evenly items-center gap-3 grow">
    <div v-if="roll !== 0 && isMyTurn" class="text-xl text-indigo-400">Roll: {{ roll }}</div>
    <button @click="store.fetchState"
      class="px-6 py-2 bg-slate-800 hover:bg-slate-700 rounded-md border border-slate-700 transition-colors">
      Sync
    </button>
    <button
      class="px-6 py-2 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-md shadow-lg shadow-indigo-500/20 transition-transform active:scale-95"
      @click="store.rollDice"
      :disabled="store.isRolling || roll !== 0 || !isMyTurn || store.doneMoving || playerCount < 2">
      <span v-if="store.isRolling && roll === 0">Rolling...</span>
      <span v-else-if="roll !== 0">Waiting for {{ store.gameState?.current_turn }}</span>
      <span v-else>Roll Dice</span>
    </button>
    <button @click="handleDraw"
      class="px-6 py-2 bg-slate-800 hover:bg-slate-700 rounded-md border border-slate-700 transition-colors" :disabled="store.isDrawing ||
        !isMyTurn ||
        handFull ||
        store.donePlaying ||
        !store.doneMoving ||
        playerCount < 2
        ">
      <span v-if="store.isDrawing && !handFull">Drawing...</span>
      <span v-else-if="handFull">Hand Full</span>
      <span v-else-if="store.donePlaying || !store.doneMoving">Cannot Draw</span>
      <span v-else>Refill Hand</span>
    </button>
    <button @click="store.endTurn"
      class="px-6 py-2 bg-red-500 hover:bg-red-400 rounded-md border border-red-400 transition-colors" :disabled="!isMyTurn ||
        roll !== 0 ||
        store.isDrawing ||
        store.isRolling ||
        !store.doneMoving ||
        !store.donePlaying ||
        playerCount < 2
        ">
      End Turn
    </button>
    <button @click="store.leaveGame"
      class="px-6 py-2 bg-red-500 hover:bg-red-400 rounded-md border border-red-400 transition-colors">
      Leave Game
    </button>
  </div>
</template>
