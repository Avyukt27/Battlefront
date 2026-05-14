export enum Status {
  Fracture,
  Poison,
}

export enum PlayerClass {
  Gunslinger,
  Arsenist,
  Mage,
  Knight,
  Assassin,
}

export type CardAbility = { DamageMul: { multiplier: number; threshold: number } } | 'ShieldPierce';

export type CardEffect =
  | { Damage: { power: number } }
  | { Heal: { amount: number } }
  | { SkillCheck: { threshold: number } }
  | { ApplyStatus: { status: Status; duration: number } }
  | { CureStatus: { status: Status } }
  | { Range: { max_range: number } }
  | { Shield: { value: number } }
  | { Ability: { ability: CardAbility; cooldown: number } };

export interface Card {
  id: string;
  name: string;
  isSignature: boolean;
  cooldown: number;
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
  maxHealth: number;
  shield: number;
  statusEffects: ActiveEffect[];
  class: PlayerClass;
  cards: Card[];
}

export interface GameState {
  players: Player[];
  currentTurn: string;
  lastRoll: number;
  width: number;
  height: number;
}
