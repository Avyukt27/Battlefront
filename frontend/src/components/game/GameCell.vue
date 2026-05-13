<script setup lang="ts">
import { computed } from 'vue';
import { useGameStore } from '@/stores/game';

const props = defineProps<{ x: number; y: number }>();
const store = useGameStore();

const player = computed(() =>
  store.gameState?.players.find((p) => p.x === props.x && p.y === props.y),
);
const myPlayer = computed(() => store.gameState?.players.find((p) => p.id === store.myPlayerId));

const isCurrentTurn = computed(() => store.gameState?.current_turn === player.value?.colour);

const isReachable = computed(() => {
  const state = store.gameState;
  if (!state || !myPlayer.value || state.last_roll === 0) return false;
  if (state.current_turn !== myPlayer.value.colour) return false;

  const dx = Math.abs(myPlayer.value.x - props.x);
  const dy = Math.abs(myPlayer.value.y - props.y);
  const distance = dx + dy;
  return distance > 0 && distance <= state.last_roll;
});

const isTargetable = computed(() => {
  if (!store.selectedCardId || !store.gameState || !myPlayer.value) return false;

  const card = myPlayer.value.cards.find((c) => c.id === store.selectedCardId);
  if (!card) return false;

  const rangeEffect = card.effects.find((e) => 'Range' in e);
  if (rangeEffect && 'Range' in rangeEffect) {
    const range = rangeEffect.Range.max_range;
    const startX = myPlayer.value.x;
    const startY = myPlayer.value.y;
    const dist = Math.abs(startX - props.x) + Math.abs(startY - props.y);

    if (dist === 0 && range !== 0) return false;
    return dist <= range;
  }

  return false;
});

const handleMove = () => {
  if (store.myPlayerId === null || store.isDrawing || store.isRolling) {
    return;
  }

  if (store.selectedCardId) {
    store.useCard(props.x, props.y);
  } else {
    store.makeMove(store.myPlayerId, props.x, props.y);
  }
};
</script>

<template>
  <div @click="handleMove"
    class="relative w-10 h-10 sm:w-14 sm:h-14 bg-slate-800/40 border border-slate-700/10 hover:bg-slate-700/60 transition-all cursor-pointer flex items-center justify-center overflow-hidden"
    :class="{
      'bg-red-500/20 border-red-500/50 shadow-[inset_0_0_15px_rgba(239,68,68,0.4)]': isTargetable,
      'bg-indigo-500/20 border-indigo-500/50 shadow-[inset_0_0_15px_rgba(99,102,241,0.2)]':
        isReachable && !store.selectedCardId,
      'bg-slate-800/40 border-slate-700/10':
        !isTargetable && !(isReachable && !store.selectedCardId),
    }">
    <div v-if="isReachable && !player" class="w-2 h-2 rounded-full bg-indigo-400/40"></div>
    <div v-if="player" class="w-4/5 h-4/5 rounded-full shadow-2xl transition-all duration-500 transform scale-90 z-10"
      :class="{
        'bg-red-600 border-2 border-red-400': player.colour === 'Red',
        'bg-blue-600 border-2 border-blue-400': player.colour === 'Blue',
        'bg-green-600 border-2 border-green-400': player.colour === 'Green',
      }">
      <div v-if="isCurrentTurn" class="absolute inset-0 rounded-full animate-ping bg-white/30"></div>
    </div>

    <span class="absolute bottom-0.5 right-1 text-[8px] text-slate-700 select-none">
      {{ x }},{{ y }}
    </span>
  </div>
</template>
