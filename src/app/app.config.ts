import { ApplicationConfig, Component, inject, provideAppInitializer } from '@angular/core';
import { provideRouter } from '@angular/router';
import { LazyStore } from '@tauri-apps/plugin-store';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';

import { routes } from './app.routes';
import { DroneService } from './service/drone.service';
import { TuningService } from './service/tuning.service';

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    provideAnimationsAsync(),
    {
      provide: LazyStore,
      useValue: new LazyStore('settings.json', { autoSave: true }),
    },
    provideAppInitializer(() => {
      const ts = inject(TuningService);
      const ds = inject(DroneService);

      return ts.init().then(() => ds.init());
    }),
  ],
};
