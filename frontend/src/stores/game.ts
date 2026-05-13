import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { GameState } from './models';
import { gameApi } from '@/api/gameClient';

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
  const lastError = ref<string | null>(null);

  function setError(msg: string) {
    lastError.value = msg;
    setTimeout(() => {
      lastError.value = null;
    }, 4000);
  }

  function handleActionError(err: unknown) {
    if (err instanceof Error) {
      setError(err.message);
    } else {
      setError(String(err));
    }
  }

  async function createGame() {
    try {
      const data = await gameApi.createGame();
      gameId.value = data.game_id;
    } catch (err) {
      setError('Failed to create game');
    }
  }

  async function joinGame(id: string, className: string) {
    try {
      const data = await gameApi.joinGame(id, className);
      myPlayerId.value = data.player_id;
      gameId.value = id;
      gameState.value = data.state;
      localStorage.setItem('saved_game_id', id);
      localStorage.setItem('saved_player_id', data.player_id.toString());
    } catch (err) {
      handleActionError(err);
    }
  }

  async function fetchState() {
    if (!gameId.value) return;
    try {
      gameState.value = await gameApi.fetchState(gameId.value);
    } catch (err) {
      leaveGame();
    }
  }

  async function makeMove(playerId: number, x: number, y: number) {
    if (!gameId.value) return;
    try {
      gameState.value = await gameApi.makeMove(gameId.value, {
        player_id: playerId,
        target_x: x,
        target_y: y,
      });
    } catch (err) {
      handleActionError(err);
    }
  }

  async function rollDice() {
    if (isRolling.value || !gameId.value) return;
    isRolling.value = true;
    try {
      gameState.value = await gameApi.rollDice(gameId.value);
    } catch (err) {
      handleActionError(err);
    } finally {
      isRolling.value = false;
    }
  }

  async function useCard(targetX: number, targetY: number) {
    if (!selectedCardId.value || !gameId.value) return;
    try {
      gameState.value = await gameApi.useCard(gameId.value, {
        card_id: selectedCardId.value,
        attacker_id: myPlayerId.value!,
        target_pos: [targetX, targetY],
      });
      selectedCardId.value = null;
    } catch (err) {
      handleActionError(err);
    }
  }

  async function drawCard() {
    if (isDrawing.value || !gameId.value || !myPlayerId.value) return;
    isDrawing.value = true;
    try {
      gameState.value = await gameApi.drawCard(gameId.value, myPlayerId.value);
    } catch (err) {
      handleActionError(err);
    } finally {
      isDrawing.value = false;
    }
  }

  async function endTurn() {
    if (!gameId.value || !myPlayerId.value) return;
    try {
      gameState.value = await gameApi.endTurn(gameId.value, myPlayerId.value);
    } catch (err) {
      handleActionError(err);
    }
  }

  function leaveGame() {
    gameId.value = null;
    myPlayerId.value = null;
    gameState.value = null;
    localStorage.clear();
    window.location.reload();
  }

  function selectCard(cardId: string) {
    selectedCardId.value = selectedCardId.value === cardId ? null : cardId;
  }

  return {
    gameState,
    gameId,
    myPlayerId,
    isRolling,
    isDrawing,
    selectedCardId,
    lastError,
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
