// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type {Chord} from './Chord';
import type {Instrument} from './Instrument';

export type Drone = {
  midi: number;
  tuning: number;
  instrument: Instrument;
  chord: Chord;
  chorus: boolean;
};
