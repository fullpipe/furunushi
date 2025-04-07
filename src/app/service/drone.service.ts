import {effect, Injectable, signal} from '@angular/core';
import {LazyStore} from '@tauri-apps/plugin-store';
import {Drone} from '../bindings/Drone';
import {invoke} from '@tauri-apps/api/core';
import {TuningService} from './tuning.service';

export const DEFAULT_TUNING = 440.0;
export const DEFAULT_DRONE = {
  midi: 49,
  tuning: DEFAULT_TUNING,
  instrument: 'Organ',
  chord: 'Pure',
  chorus: true,
} as Drone;

@Injectable({
  providedIn: 'root',
})
export class DroneService {
  private _playing = signal(false);

  public drone: Drone = DEFAULT_DRONE;
  public playing = this._playing.asReadonly();

  constructor(
    private store: LazyStore,
    private tuning: TuningService,
  ) {
    effect(() => {
      this.drone.tuning = this.tuning.tuning();
    });
  }

  async init() {
    this.drone = (await this.store.get<Drone>('drone')) || DEFAULT_DRONE;
    this.drone.tuning = this.tuning.tuning();
  }

  async set(drone: Drone) {
    this.drone = drone;
    this.drone.tuning = this.tuning.tuning();
    await this.store.set('drone', this.drone);

    if (this.playing()) {
      await this.play();
    }
  }

  async play() {
    await invoke('drone_play', {drone: this.drone});
    this._playing.set(true);
  }

  async stop() {
    await invoke('drone_pause');
    this._playing.set(false);
  }
}
