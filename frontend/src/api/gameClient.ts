const BASE_URL = 'http://localhost:3000/api';

export const gameApi = {
  async createGame() {
    const res = await fetch(`${BASE_URL}/create`, { method: 'POST' });
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },

  async joinGame(gameId: string) {
    const res = await fetch(`${BASE_URL}/join/${gameId}`, { method: 'POST' });
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },

  async fetchState(gameId: string) {
    const res = await fetch(`${BASE_URL}/state/${gameId}`);
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },

  async makeMove(
    gameId: string,
    payload: { player_id: number; target_x: number; target_y: number },
  ) {
    const res = await fetch(`${BASE_URL}/move/${gameId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },

  async rollDice(gameId: string) {
    const res = await fetch(`${BASE_URL}/roll/${gameId}`, { method: 'POST' });
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },

  async useCard(
    gameId: string,
    payload: { card_id: string; attacker_id: number; target_pos: [number, number] },
  ) {
    const res = await fetch(`${BASE_URL}/use/${gameId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },

  async drawCard(gameId: string, payload: { player_id: number }) {
    const res = await fetch(`${BASE_URL}/draw/${gameId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },

  async endTurn(gameId: string, payload: { player_id: number }) {
    const res = await fetch(`${BASE_URL}/end_turn/${gameId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },

  async leaveGame(gameId: string, payload: { player_id: number }) {
    const res = await fetch(`${BASE_URL}/leave/${gameId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  },
};
