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
  const gameId = ref<string | null>(null);
  const myPlayerId = ref<number | null>(null);
  const isRolling = ref(false);

  async function createGame() {
    const response = await fetch('http://localhost:3000/api/create', {
      method: 'POST',
    });
    if (response.ok) {
      const data = await response.json();
      gameId.value = data.game_id;
    }
  }

  async function joinGame(id: string, colour: string, className: string) {
    const response = await fetch(`http://localhost:3000/api/join/${id}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ colour, class: className }),
    });

    if (response.ok) {
      const data = await response.json();
      myPlayerId.value = data.your_id;
      gameId.value = id;
      gameState.value = data.state;
    } else {
      alert('Could not join game. Check the ID.');
    }
  }

  async function fetchState() {
    if (!gameId.value) return;
    const response = await fetch(`http://localhost:3000/api/state/${gameId.value}`);
    if (response.ok) {
      gameState.value = await response.json();
    }
  }

  async function makeMove(playerId: number, x: number, y: number) {
    if (!gameId.value) return;

    const response = await fetch(`http://localhost:3000/api/move/${gameId.value}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ player_id: playerId, target_x: x, target_y: y }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.warn('Move rejected:', errorText);
      return;
    }

    gameState.value = await response.json();
  }

  async function rollDice() {
    if (isRolling.value || !gameId.value) return;

    isRolling.value = true;
    try {
      const response = await fetch(`http://localhost:3000/api/roll/${gameId.value}`, {
        method: 'POST',
      });
      if (response.ok) {
        gameState.value = await response.json();
      }
    } finally {
      isRolling.value = false;
    }
  }

  return {
    gameState,
    gameId,
    myPlayerId,
    isRolling,
    createGame,
    joinGame,
    fetchState,
    makeMove,
    rollDice,
  };
});
