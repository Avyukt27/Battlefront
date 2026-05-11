<script setup lang="ts">
import GameCell from './GameCell.vue';
</script>

<template>
  <div class="grid grid-cols-8 gap-1 bg-slate-900 p-1.5 border border-slate-800 rounded-lg shadow-2xl">
    <template v-for="y in 8" :key="`row-${y}`">
      <GameCell v-for="x in 8" :key="`cell-${x - 1}-${y - 1}`" :x="x - 1" :y="y - 1" />
    </template>
  </div>
</template>
