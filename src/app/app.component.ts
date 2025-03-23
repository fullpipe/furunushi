import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, RouterOutlet } from '@angular/router';
import { Subject, merge, debounceTime, tap, distinctUntilChanged, fromEvent } from 'rxjs';

@Component({
  selector: 'app-root',
  imports: [CommonModule, RouterOutlet, RouterModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  private reset$ = new Subject<boolean>();
  public readonly isActive$ = merge(
    fromEvent(document, 'mousemove'),
    fromEvent(document, 'touchstart'),
    this.reset$.pipe(debounceTime(5000))
  ).pipe(
    tap(() => this.reset$.next(false)),
    distinctUntilChanged()
  );

  ngOnInit(): void {}
}
