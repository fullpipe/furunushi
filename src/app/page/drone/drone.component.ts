import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { Drone } from '../../bindings/Drone';
import { LazyStore } from '@tauri-apps/plugin-store';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';
import { JsonPipe } from '@angular/common';
import { Subscription } from 'rxjs';
import { DroneService } from '../../service/drone.service';

@Component({
  selector: 'app-drone',
  imports: [FormsModule, JsonPipe, ReactiveFormsModule],
  templateUrl: './drone.component.html',
  styleUrl: './drone.component.scss',
})
export class DroneComponent {
  ss = new Subscription();
  droneForm = new FormGroup({
    midi: new FormControl(50, [Validators.required, Validators.min(1), Validators.max(88)]),
    tuning: new FormControl(440.0, [Validators.required, Validators.min(300), Validators.max(500)]),
    instrument: new FormControl('Organ', [Validators.required]),
    chord: new FormControl('Minor', [Validators.required]),
    chorus: new FormControl(true, [Validators.required]),
  });

  constructor(public drone: DroneService) {
    this.droneForm.setValue(this.drone.drone);
    this.ss.add(this.droneForm.valueChanges.subscribe((d) => this.drone.set(d as Drone)));
  }

  async play() {
    await this.drone.play();
  }

  async stop() {
    await this.drone.stop();
  }

  ngOnDestroy(): void {
    this.ss.unsubscribe();
  }
}
