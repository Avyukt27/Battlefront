<script setup lang="ts">
import { useGameStore } from '@/stores/game';

const store = useGameStore();
</script>

<template>
  <aside class="flex flex-col gap-2 p-4 bg-slate-900/80 border-r border-slate-800 w-64">
    <h3 class="text-xs font-bold text-slate-500 uppercase tracking-widest mb-2">Players</h3>

    <div v-for="player in store.gameState?.players" :key="player.id" :class="[
      'p-3 rounded-lg border transition-all',
      store.gameState?.currentTurn === player.colour
        ? 'bg-indigo-500/20 border-indigo-500 shadow-[0_0_15px_rgba(99,102,241,0.3)]'
        : 'bg-slate-800/40 border-transparent opacity-70',
    ]">
      <div class="flex justify-between items-center mb-2">
        <span class="font-bold" :style="{ color: player.colour }">{{ player.class }}</span>
        <span class="text-xs font-mono">{{ player.health }} / {{ player.maxHealth }}</span>
      </div>
      <div class="flex flex-row justify-between">
        <span class="text-xs font-mono">Shield: {{ player.shield }}</span>
        <p v-if="player.id === store.myPlayerId" class="text-[10px] text-indigo-400 mt-1 uppercase font-bold">
          [You]
        </p>
      </div>
    </div>
  </aside>
</template>
