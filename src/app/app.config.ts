import {
  ApplicationConfig,
  inject,
  provideAppInitializer,
  provideZonelessChangeDetection,
} from '@angular/core';
import {provideRouter} from '@angular/router';
import {LazyStore} from '@tauri-apps/plugin-store';

import {routes} from './app.routes';
import {DroneService} from './service/drone.service';
import {TuningService} from './service/tuning.service';

export const appConfig: ApplicationConfig = {
  providers: [
    provideZonelessChangeDetection(),
    provideRouter(routes),
    {
      provide: LazyStore,
      useValue: new LazyStore('settings.json'),
    },
    provideAppInitializer(() => {
      const ts = inject(TuningService);
      const ds = inject(DroneService);

      return ts.init().then(() => ds.init());
    }),
  ],
};
