import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { GameState } from './models';

export const useGameStore = defineStore('game', () => {
  const gameState = ref<GameState | null>(null);
  const gameId = ref<string | null>(localStorage.getItem('saved_game_id'));
  const myPlayerId = ref<number | null>(
    localStorage.getItem('saved_player_id')
      ? Number(localStorage.getItem('saved_player_id'))
      : null,
  );
  const isRolling = ref(false);
  const isDrawing = ref(false);
  const selectedCardId = ref<string | null>(null);
  const doneMoving = ref(false);
  const donePlaying = ref(false);

  async function createGame() {
    const response = await fetch('http://localhost:3000/api/create', {
      method: 'POST',
    });
    if (response.ok) {
      const data = await response.json();
      gameId.value = data.game_id;
    }
  }

  async function joinGame(id: string, className: string) {
    const response = await fetch(`http://localhost:3000/api/join/${id}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ class: className }),
    });

    if (response.ok) {
      const data = await response.json();
      myPlayerId.value = data.player_id;
      gameId.value = id;
      localStorage.setItem('saved_game_id', id);
      localStorage.setItem('saved_player_id', data.player_id);
      gameState.value = data.state;
    } else {
      alert('Could not join game. Check the ID.');
    }
  }

  async function fetchState() {
    if (!gameId.value) return;
    try {
      const response = await fetch(`http://localhost:3000/api/state/${gameId.value}`);
      if (response.ok) {
        gameState.value = await response.json();
      } else if (response.status === 404) {
        console.warn('Room not found, clearing session...');
        leaveGame();
      }
    } catch (err) {
      console.error('Network error during sync:', err);
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

    doneMoving.value = true;
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

  function leaveGame() {
    gameId.value = null;
    myPlayerId.value = null;
    gameState.value = null;
    localStorage.removeItem('saved_game_id');
    localStorage.removeItem('saved_player_id');
    window.location.reload();
  }

  function selectCard(cardId: string) {
    if (selectedCardId.value === cardId) {
      selectedCardId.value = null;
    } else {
      selectedCardId.value = cardId;
    }
  }

  async function useCard(targetX: number, targetY: number) {
    if (!selectedCardId.value) return;

    const response = await fetch(`http://localhost:3000/api/use/${gameId.value}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        card_id: selectedCardId.value,
        attacker_id: myPlayerId.value,
        target_pos: [targetX, targetY],
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.warn('Card rejected:', errorText);
      return;
    }

    selectedCardId.value = null;
    donePlaying.value = true;
    gameState.value = await response.json();
  }

  async function drawCard() {
    if (isDrawing.value || !gameId.value) return;
    isDrawing.value = true;

    console.log(
      `Sending request to http://localhost:3000/api/draw/${gameId.value}/${myPlayerId.value}`,
    );

    try {
      const response = await fetch(
        `http://localhost:3000/api/draw/${gameId.value}/${myPlayerId.value}`,
        { method: 'POST' },
      );
      if (response.ok) {
        donePlaying.value = true;
        gameState.value = await response.json();
      }
    } finally {
      isDrawing.value = false;
    }
  }

  async function endTurn() {
    const response = await fetch(
      `http://localhost:3000/api/end_turn/${gameId.value}/${myPlayerId.value}`,
      { method: 'POST' },
    );

    if (!response.ok) {
      const errorText = await response.text();
      console.warn('Could not end turn:', errorText);
      return;
    }

    doneMoving.value = false;
    donePlaying.value = false;
    gameState.value = await response.json();
  }

  return {
    gameState,
    gameId,
    myPlayerId,
    isRolling,
    isDrawing,
    doneMoving,
    donePlaying,
    selectedCardId,
    createGame,
    joinGame,
    fetchState,
    makeMove,
    rollDice,
    leaveGame,
    selectCard,
    useCard,
    drawCard,
    endTurn,
  };
});
