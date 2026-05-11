import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useGameStore = defineStore('game', () => {
  const gameState = ref(null);

  async function fetchState() {
    const response = await fetch('http://localhost:3000/api/state');
    gameState.value = await response.json();
  }

  async function makeMove(playerId: number, x: number, y: number) {
    const response = await fetch('http://localhost:3000/api/move', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ player_id: playerId, target_x: x, target_y: y }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      alert(errorText);
      return;
    }

    gameState.value = await response.json();
  }

  return { gameState, fetchState, makeMove };
});
