import { Routes } from '@angular/router';
import { TunerComponent } from './page/tuner/tuner.component';
import { MirrorComponent } from './page/mirror/mirror.component';

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
    path: '**',
    redirectTo: 'tuner',
  },
];
