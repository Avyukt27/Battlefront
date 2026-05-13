<script setup lang="ts">
import { useGameStore } from '@/stores/game';
import { useParallax } from '@vueuse/core';
import { computed, reactive, useTemplateRef, type CSSProperties } from 'vue';

const props = defineProps<{ id: string; name: string }>();

const store = useGameStore();
const target = useTemplateRef('card');
const parallax = reactive(useParallax(target));

const myPlayer = computed(() => store.gameState?.players.find((p) => p.id === store.myPlayerId));
const isSelected = computed(() => store.selectedCardId === props.id);

const containerStyle: CSSProperties = {
  perspective: '1000px',
};

const cardBase = computed(() => ({
  background: '#fff',
  width: '180px',
  height: '270px',
  borderRadius: '12px',
  border: '1px solid #cdcdcd',
  backfaceVisibility: 'hidden' as const,
  transformStyle: 'preserve-3d' as const,
  transition: '.3s ease-out all',
  boxShadow: '0 0 20px 0 rgba(255, 255, 255, 0.25)',
}));

const cardActive = computed(() => ({
  ...cardBase.value,
  transform: `rotateX(${parallax.roll * 10}deg) rotateY(${parallax.tilt * 20}deg)`,
  border: '2px solid rgba(49, 65, 88, 0.6)',
}));

const handleSelect = () => {
  if (!store.doneMoving || !myPlayer.value || !store.gameState || store.donePlaying) return;
  if (!(myPlayer.value.colour === store.gameState.current_turn)) return;

  store.selectCard(props.id);
};
</script>

<template>
  <div ref="card" class="ease-outduration<300> transition-all">
    <div :style="containerStyle">
      <div :style="[isSelected ? cardActive : cardBase]" @click="handleSelect">
        <img :src="`/cards/${name}.png`" :alt="name" />
      </div>
    </div>
  </div>
</template>
