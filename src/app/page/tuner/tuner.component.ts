import { Component, effect, signal } from '@angular/core';
import { AsyncPipe, CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DeviationComponent } from '../../component/deviation/deviation.component';
import { NoteComponent } from '../../component/note/note.component';
import { TunerService } from '../../service/tuner.service';
import { DroneService } from '../../service/drone.service';
import { TuningService } from '../../service/tuning.service';

@Component({
  selector: 'app-tuner',
  imports: [AsyncPipe, CommonModule, DeviationComponent, FormsModule, NoteComponent],
  templateUrl: './tuner.component.html',
  styleUrl: './tuner.component.scss',
})
export class TunerComponent {
  tuning = signal(this.tuningService.tuning());

  constructor(public tuner: TunerService, private drone: DroneService, private tuningService: TuningService) {
    effect(() => {
      this.tuningService.set(this.tuning());
    });
  }

  async ngOnInit() {
    await this.drone.stop();
  }
}
