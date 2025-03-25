import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, RouterOutlet } from '@angular/router';
import { Subject, merge, debounceTime, tap, distinctUntilChanged, fromEvent, map, of } from 'rxjs';
import { DroneService } from './service/drone.service';

@Component({
  selector: 'app-root',
  imports: [CommonModule, RouterOutlet, RouterModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
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

  ngOnInit(): void {}
}
