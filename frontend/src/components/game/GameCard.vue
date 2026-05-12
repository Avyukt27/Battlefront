<script setup lang="ts">
import { useParallax } from '@vueuse/core';
import { computed, reactive, ref, useTemplateRef, type CSSProperties } from 'vue';

const props = defineProps<{ key: string; card: string }>();

const target = useTemplateRef('card');
const parallax = reactive(useParallax(target));

const isActive = ref(false);

const containerStyle: CSSProperties = {
  perspective: '600px',
};

const cardBase = computed(() => ({
  background: '#fff',
  width: '7rem',
  height: '9rem',
  borderRadius: '5px',
  border: '1px solid #cdcdcd',
  overflow: 'hidden',
  transition: '.3s ease-out all',
  boxShadow: '0 0 20px 0 rgba(255, 255, 255, 0.25)',
}));

const cardActive = computed(() => ({
  ...cardBase.value,
  transform: `rotateX(${parallax.roll * 10}deg) rotateY(${parallax.tilt * 20}deg)`,
  border: '2px solid rgba(49, 65, 88, 0.6)',
}));

function changeState() {
  isActive.value = !isActive.value;
}
</script>

<template>
  <div ref="card" class="ease-outduration<300> transition-all">
    <div :style="containerStyle">
      <div :style="[isActive ? cardActive : cardBase]" @click="changeState">
        <img src="https://jaromvogel.com/images/design/jumping_rabbit/page2layer0.png" :alt="card" />
      </div>
    </div>
  </div>
</template>
