import { Injectable, signal } from '@angular/core';
import { LazyStore } from '@tauri-apps/plugin-store';

export const DEFAULT_TUNING = 440.0;

@Injectable({
  providedIn: 'root',
})
export class TuningService {
  private _tuning = signal(DEFAULT_TUNING);
  public tuning = this._tuning.asReadonly();

  constructor(private store: LazyStore) {}

  async init() {
    await this._tuning.set((await this.store.get<number>('tuning')) || DEFAULT_TUNING);
  }

  async set(tuning: number) {
    await this._tuning.set(tuning);
    await this.store.set('tuning', tuning);
  }
}
