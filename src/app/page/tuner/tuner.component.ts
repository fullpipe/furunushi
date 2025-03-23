import { Component, effect, signal } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { AsyncPipe, CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DeviationComponent } from '../../component/deviation/deviation.component';
import { NoteComponent } from '../../component/note/note.component';
import { LazyStore } from '@tauri-apps/plugin-store';
import { TunerService } from '../../service/tuner.service';

@Component({
  selector: 'app-tuner',
  imports: [AsyncPipe, CommonModule, DeviationComponent, FormsModule, NoteComponent],
  templateUrl: './tuner.component.html',
  styleUrl: './tuner.component.scss',
})
export class TunerComponent {
  tuning = signal(440);

  constructor(private store: LazyStore, public tuner: TunerService) {
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
}
