import { ApplicationConfig, Component } from '@angular/core';
import { provideRouter } from '@angular/router';
import { LazyStore } from '@tauri-apps/plugin-store';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';

import { routes } from './app.routes';

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    provideAnimationsAsync(),
    {
      provide: LazyStore,
      useValue: new LazyStore('settings.json', { autoSave: true }),
    },
  ],
};
