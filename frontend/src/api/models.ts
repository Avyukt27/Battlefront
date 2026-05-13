export enum Status {
  Bleed,
  Poison,
}

export type CardEffect =
  | { Damage: { power: number } }
  | { Heal: { amount: number } }
  | { SkillCheck: { threshold: number } }
  | { ApplyStatus: { status: Status; duration: number } }
  | { CureStatus: { status: Status } }
  | { Range: { max_range: number } };

export interface Card {
  id: string;
  name: string;
  effects: CardEffect[];
}

export interface ActiveEffect {
  status: Status;
  duration: number;
}

export interface Player {
  id: number;
  colour: 'Red' | 'Blue' | 'Green';
  x: number;
  y: number;
  health: number;
  max_health: number;
  status_effects: ActiveEffect[];
  class: string;
  cards: Card[];
}

export interface GameState {
  players: Player[];
  current_turn: string;
  last_roll: number;
  width: number;
  height: number;
}
