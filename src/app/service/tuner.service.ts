import { Injectable } from '@angular/core';
import { Subject, merge, debounceTime, tap, pairwise, map, distinctUntilChanged } from 'rxjs';
import { Note } from '../bindings/Note';
import listen from '../tools/listen';
import { invoke } from '@tauri-apps/api/core';

const SMOOTH_FACTOR = 0.25;

@Injectable({
  providedIn: 'root',
})
export class TunerService {
  private reset$ = new Subject<null>();
  private subscriptionsCount = 0;

  public readonly note$ = merge(listen<Note>('tuner::note'), this.reset$.pipe(debounceTime(1000))).pipe(
    tap(() => this.reset$.next(null)),
    pairwise(),
    map((p) => {
      if (p[0] === null || p[1] === null) {
        return p[1];
      }

      if (p[0].midi != p[1].midi) {
        return p[1];
      }

      p[1].deviation = Math.round((1 - SMOOTH_FACTOR) * p[0].deviation + SMOOTH_FACTOR * p[1].deviation);
      p[0].deviation = p[1].deviation;

      return p[1];
    }),
    distinctUntilChanged(
      (prev, curr) => curr != null && prev != null && prev.midi === curr.midi && prev.deviation === curr.deviation
    ),
    tap({
      subscribe: () => {
        if (this.subscriptionsCount === 0) {
          invoke('pd_start');
        }
        this.subscriptionsCount++;
      },
      unsubscribe: () => {
        this.subscriptionsCount++;
        if (this.subscriptionsCount === 0) {
          invoke('pd_pause');
        }
      },
    })
  );

  constructor() {}
}
