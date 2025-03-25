import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { Drone } from '../../bindings/Drone';

@Component({
  selector: 'app-drone',
  imports: [],
  templateUrl: './drone.component.html',
  styleUrl: './drone.component.scss',
})
export class DroneComponent {
  ngOnInit(): void {
    //Called after the constructor, initializing input properties, and the first call to ngOnChanges.
    //Add 'implements OnInit' to the class.
  }
  play() {
    invoke('drone_play', {
      d: {
        midi: 53,
        tuning: 440.0,
        instrument: 'Sine',
        chord: 'Minor',
        chorus: true,
      } as Drone,
    });
  }
  stop() {
    invoke('drone_pause');
  }
}
