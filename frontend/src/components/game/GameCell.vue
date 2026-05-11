<script setup lang="ts">
import { computed } from 'vue';
import { useGameStore } from '@/stores/game';

const props = defineProps<{ x: number; y: number }>();
const store = useGameStore();

const player = computed(() =>
  store.gameState?.players.find((p) => p.x === props.x && p.y === props.y),
);

const isCurrentTurn = computed(() => store.gameState?.current_turn === player.value?.colour);

const handleMove = () => {
  if (store.myPlayerId !== null) {
    store.makeMove(store.myPlayerId, props.x, props.y);
  } else {
    console.warn('Cannot move: No Player ID assigned.');
  }
};
</script>

<template>
  <div @click="handleMove"
    class="relative w-12 h-12 sm:w-16 sm:h-16 bg-slate-800/40 border border-slate-700/10 hover:bg-slate-700/60 transition-all cursor-pointer flex items-center justify-center overflow-hidden">
    <div v-if="player" class="w-4/5 h-4/5 rounded-full shadow-2xl transition-all duration-500 transform scale-90 z-10"
      :class="player.colour === 'Red'
          ? 'bg-red-600 border-2 border-red-400'
          : 'bg-blue-600 border-2 border-blue-400'
        ">
      <div v-if="isCurrentTurn" class="absolute inset-0 rounded-full animate-ping bg-white/30"></div>
    </div>

    <span class="absolute bottom-0.5 right-1 text-[8px] text-slate-700 select-none">
      {{ x }},{{ y }}
    </span>
  </div>
</template>
