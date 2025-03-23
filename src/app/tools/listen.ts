import { listen as tauriListen } from '@tauri-apps/api/event';
import { Observable } from 'rxjs';

export default function listen<T>(eventName: string): Observable<T> {
  return new Observable((subscriber) => {
    const unlisten = tauriListen<T>(eventName, (v) => subscriber.next(v.payload));

    return async () => {
      (await unlisten)();
    };
  });
}
