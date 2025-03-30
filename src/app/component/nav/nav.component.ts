import { Component } from '@angular/core';
import { Subject, merge, fromEvent, map, debounceTime, of, tap, distinctUntilChanged } from 'rxjs';
import { DroneService } from '../../service/drone.service';
import { RouterModule } from '@angular/router';
import { AsyncPipe } from '@angular/common';

@Component({
  selector: 'app-nav',
  imports: [RouterModule, AsyncPipe],
  templateUrl: './nav.component.html',
  styleUrl: './nav.component.scss',
})
export class NavComponent {
  private reset$ = new Subject<boolean>();
  public readonly isActive$ = merge(
    fromEvent(document, 'mousemove').pipe(map(() => true)),
    fromEvent(document, 'touchstart').pipe(map(() => true)),
    this.reset$.pipe(debounceTime(5000)),
    of(true)
  ).pipe(
    tap(() => this.reset$.next(false)),
    distinctUntilChanged()
  );

  constructor(public drone: DroneService) {}
}
