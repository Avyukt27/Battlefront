const BASE_URL = 'http://localhost:3000/api';

export const gameApi = {
  async fetchState(gameId: string) {
    const res = await fetch(`${BASE_URL}/state/${gameId}`);
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

  // ... add move, roll, draw, etc.
};
