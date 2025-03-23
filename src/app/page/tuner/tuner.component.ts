import { Component, effect, OnDestroy, signal } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { Note } from '../../bindings/Note';
import listen from '../../tools/listen';
import { AsyncPipe, CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { debounceTime, distinctUntilChanged, map, merge, pairwise, Subject, Subscription, tap } from 'rxjs';
import { DeviationComponent } from '../../component/deviation/deviation.component';
import { NoteComponent } from '../../component/note/note.component';
import { LazyStore } from '@tauri-apps/plugin-store';

const SMOOTH_FACTOR = 0.25;

@Component({
  selector: 'app-tuner',
  imports: [AsyncPipe, CommonModule, DeviationComponent, FormsModule, NoteComponent],
  templateUrl: './tuner.component.html',
  styleUrl: './tuner.component.scss',
})
export class TunerComponent implements OnDestroy {
  ss = new Subscription();

  tuning = signal(440);

  reset$ = new Subject<null>();
  note$ = merge(listen<Note>('tuner::note'), this.reset$.pipe(debounceTime(1000))).pipe(
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
    )
  );

  constructor(private store: LazyStore) {
    this.store.get<number>('tuning').then((oldTuning) => {
      if (oldTuning) {
        this.tuning.set(oldTuning);
      }
    });

    let init = true;

    effect(() => {
      invoke('pd_base', { f: this.tuning() });

      if (!init) {
        this.store.set('tuning', this.tuning());
      }

      init = false;
    });
  }

  updateTuning(f: number) {}

  async ngOnInit() {
    invoke('pd_start');
  }

  ngOnDestroy(): void {
    invoke('pd_pause');
    this.ss.unsubscribe();
  }
}
