import {Routes} from '@angular/router';
import {TunerComponent} from './page/tuner/tuner.component';
import {MirrorComponent} from './page/mirror/mirror.component';
import {DroneComponent} from './page/drone/drone.component';

export const routes: Routes = [
  {
    path: 'mirror',
    component: MirrorComponent,
  },
  {
    path: 'tuner',
    component: TunerComponent,
  },
  {
    path: 'drone',
    component: DroneComponent,
  },
  {
    path: '**',
    redirectTo: 'tuner',
  },
];
