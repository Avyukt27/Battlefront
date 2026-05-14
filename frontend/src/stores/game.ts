import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { GameState } from '@/api/models';
import { gameApi } from '@/api/gameClient';

export const useGameStore = defineStore('game', () => {
  const gameState = ref<GameState | null>(null);
  const gameId = ref<string | null>(localStorage.getItem('gameId'));
  const myPlayerId = ref<number | null>(
    localStorage.getItem('playerId') ? Number(localStorage.getItem('playerId')) : null,
  );

  const isRolling = ref(false);
  const isDrawing = ref(false);
  const isUsingAbility = ref(false);
  const doneMoving = ref(false);
  const donePlaying = ref(false);
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
      gameId.value = data.gameId;
    } catch (err) {
      setError('Failed to create game');
    }
  }

  async function joinGame(id: string) {
    try {
      const data = await gameApi.joinGame(id);
      myPlayerId.value = data.playerId;
      gameId.value = id;
      gameState.value = data.state;
      localStorage.setItem('gameId', id);
      localStorage.setItem('playerId', data.playerId.toString());
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
        playerId: playerId,
        targetX: x,
        targetY: y,
      });
      doneMoving.value = true;
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
      const data = await gameApi.useCard(gameId.value, {
        cardId: selectedCardId.value,
        attackerId: myPlayerId.value!,
        targetPos: [targetX, targetY],
        useAbility: isUsingAbility.value,
      });
      gameState.value = data[0];
      selectedCardId.value = null;
      donePlaying.value = true;
      isUsingAbility.value = false;

      if (!data[1]) {
        setError('Missed');
      }
    } catch (err) {
      handleActionError(err);
    }
  }

  async function drawCard() {
    if (isDrawing.value || !gameId.value || !myPlayerId.value) return;
    isDrawing.value = true;
    try {
      gameState.value = await gameApi.drawCard(gameId.value, { playerId: myPlayerId.value });
      donePlaying.value = true;
    } catch (err) {
      handleActionError(err);
    } finally {
      isDrawing.value = false;
    }
  }

  async function endTurn() {
    if (!gameId.value || !myPlayerId.value) return;
    try {
      gameState.value = await gameApi.endTurn(gameId.value, { playerId: myPlayerId.value });
      doneMoving.value = false;
      donePlaying.value = false;
      isUsingAbility.value = false;
    } catch (err) {
      handleActionError(err);
    }
  }

  async function leaveGame() {
    if (gameId.value && myPlayerId.value) {
      try {
        await gameApi.leaveGame(gameId.value, {
          playerId: myPlayerId.value,
        });
      } catch (err) {
        console.error('Failed to notify server of departure:', err);
      }
    }

    gameId.value = null;
    myPlayerId.value = null;
    gameState.value = null;
    isRolling.value = false;
    isDrawing.value = false;
    isUsingAbility.value = false;
    doneMoving.value = false;
    donePlaying.value = false;

    localStorage.removeItem('gameId');
    localStorage.removeItem('playerId');
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
    isUsingAbility,
    doneMoving,
    donePlaying,
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
