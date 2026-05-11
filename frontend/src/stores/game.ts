import { defineStore } from 'pinia';
import { ref } from 'vue';

interface Player {
  id: number;
  colour: 'Red' | 'Blue' | 'Green' | 'Yellow';
  x: number;
  y: number;
}

interface GameState {
  players: Player[];
  current_turn: string;
  last_roll: number;
  width: number;
  height: number;
}

export const useGameStore = defineStore('game', () => {
  const gameState = ref<GameState | null>(null);
  const isRolling = ref(false);

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

  async function rollDice() {
    if (isRolling.value) return;

    isRolling.value = true;
    try {
      const response = await fetch('http://localhost:3000/api/roll', {
        method: 'POST',
      });
      if (response.ok) {
        gameState.value = await response.json();
      }
    } finally {
      isRolling.value = false;
    }
  }

  return { gameState, isRolling, fetchState, makeMove, rollDice };
});
