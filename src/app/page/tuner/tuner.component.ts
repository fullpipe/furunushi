import {Component, effect, signal} from '@angular/core';
import { AsyncPipe, CommonModule } from '@angular/common';
import {FormsModule} from '@angular/forms';
import {DeviationComponent} from '../../component/deviation/deviation.component';
import {NoteComponent} from '../../component/note/note.component';
import {TunerService} from '../../service/tuner.service';
import {DroneService} from '../../service/drone.service';
import {TuningService} from '../../service/tuning.service';
import {platform} from '@tauri-apps/plugin-os';

@Component({
  selector: 'app-tuner',
  imports: [
    AsyncPipe,
    CommonModule,
    DeviationComponent,
    FormsModule,
    NoteComponent,
  ],
  templateUrl: './tuner.component.html',
  styleUrl: './tuner.component.scss',
})
export class TunerComponent {
  MIN = 300;
  MAX = 500;

  tuning = signal(this.tuningService.tuning());
  platform = platform();
  selects = Array(this.MAX - this.MIN)
    .fill(this.MIN)
    .map((x, y) => x + y);

  constructor(
    public tuner: TunerService,
    private drone: DroneService,
    private tuningService: TuningService,
  ) {
    effect(() => {
      this.tuningService.set(this.tuning()).catch(console.error);
    });
  }

  async ngOnInit() {
    await this.drone.stop();
  }
}
