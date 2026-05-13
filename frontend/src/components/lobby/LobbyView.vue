<script setup lang="ts">
import { ref } from 'vue';
import { useGameStore } from '@/stores/game';

const store = useGameStore();
const joinId = ref('');

const handleCreate = async () => {
  await store.createGame();

  if (store.gameId) {
    console.log(store.gameId);
    await store.joinGame(store.gameId);
  }
};

const handleJoin = async () => {
  if (!joinId.value) return;
  await store.joinGame(joinId.value.toLowerCase());
};
</script>

<template>
  <div class="flex items-center justify-center p-6">
    <div class="w-full max-w-md p-8 bg-slate-900 border border-slate-800 rounded-2xl shadow-2xl">
      <h2 class="text-2xl font-bold text-center mb-8 text-white uppercase tracking-widest">
        Lobby
      </h2>

      <div class="space-y-6">
        <div class="pt-4 space-y-4">
          <button @click="handleCreate"
            class="w-full py-4 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-xl transition-all shadow-lg shadow-indigo-600/20 active:scale-[0.98]">
            Create new game
          </button>

          <div class="flex items-center gap-4">
            <div class="h-px bg-slate-800 grow"></div>
            <span class="text-slate-600 text-xs font-bold">OR JOIN EXISTING</span>
            <div class="h-px bg-slate-800 grow"></div>
          </div>

          <div class="flex gap-2">
            <input v-model="joinId" placeholder="ROOM ID"
              class="grow bg-slate-950 border border-slate-800 p-4 rounded-xl text-center font-mono uppercase tracking-widest focus:border-indigo-500 outline-none" />
            <button @click="handleJoin"
              class="px-6 bg-slate-800 hover:bg-slate-700 text-white font-bold rounded-xl transition-all">
              GO
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
