<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useGameStore } from '@/stores/game';
import GameBoard from './GameBoard.vue';
import GameControls from './GameControls.vue';
import GameCard from './GameCard.vue';
import PlayerPanel from './PlayerPanel.vue';

const store = useGameStore();
const showReveal = ref(store.myPlayerId ? false : true);

const myPlayer = computed(() => store.gameState?.players.find((p) => p.id === store.myPlayerId));
const signatureCards = computed(() => myPlayer.value?.cards.filter((c) => c.is_signature) ?? []);
const inventoryCards = computed(() => myPlayer.value?.cards.filter((c) => !c.is_signature) ?? []);

onMounted(() => {
  setTimeout(() => {
    showReveal.value = false;
  }, 1500);
});
</script>

<template>
  <div class="relative flex-1 flex flex-col items-center gap-4 px-4 overflow-hidden">
    <div :class="[
      'flex-1 flex flex-col items-center gap-4 transition-all duration-1000',
      showReveal ? 'blur-xl scale-95 opacity-50' : 'blur-0 scale-100 opacity-100',
    ]">
      <div class="text-center shrink-0">
        <p class="text-slate-500 text-xs uppercase tracking-widest">Room</p>
        <p class="text-xl font-mono text-indigo-400">{{ store.gameId }}</p>
      </div>

      <div class="flex flex-row gap-4 items-stretch max-h-[60%]">
        <div class="border-r border-slate-800 pr-6 flex items-center">
          <PlayerPanel />
        </div>
        <div class="shrink shadow-2xl rounded-xl overflow-hidden border border-slate-800">
          <GameBoard />
        </div>
        <div class="border-l border-slate-800 pl-6 flex items-center w-70.25">
          <GameControls />
        </div>
      </div>

      <div class="flex flex-row gap-4 items-stretch justify-center w-full">
        <div v-if="signatureCards.length > 0"
          class="bg-orange-500/10 p-4 rounded-2xl border-2 border-orange-500/30 backdrop-blur-sm flex flex-row gap-4 items-center justify-center min-h-67.5 relative">
          <span
            class="absolute -top-3 left-4 bg-orange-600 text-[10px] font-black px-2 py-0.5 rounded uppercase text-white shadow-lg">
            Class Skill
          </span>
          <GameCard v-for="card in signatureCards" :key="card.id" :id="card.id" :name="card.name" />
        </div>

        <div
          class="bg-slate-900/50 p-4 rounded-2xl border border-slate-800 backdrop-blur-sm w-150 shrink-0 flex flex-row gap-4 items-center justify-center min-h-67.5 relative">
          <span
            class="absolute -top-3 left-4 bg-slate-800 text-[10px] font-black px-2 py-0.5 rounded uppercase text-slate-400">
            Inventory
          </span>
          <GameCard v-for="card in inventoryCards" :key="card.id" :id="card.id" :name="card.name" />
        </div>
      </div>
    </div>

    <Transition name="reveal">
      <div v-if="showReveal" class="absolute inset-0 z-100 flex items-center justify-center bg-slate-950/40">
        <div class="text-center">
          <p class="text-indigo-400 tracking-[0.4em] uppercase text-sm mb-4 animate-pulse">
            You are the...
          </p>
          <h1
            class="text-8xl font-black italic tracking-tighter text-white drop-shadow-[0_0_40px_rgba(99,102,241,0.6)]">
            {{ myPlayer?.class || '???' }}
          </h1>
        </div>
      </div>
    </Transition>
  </div>
</template>
