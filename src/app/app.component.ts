import {Component} from '@angular/core';

import {RouterModule, RouterOutlet} from '@angular/router';
import {NavComponent} from './component/nav/nav.component';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, RouterModule, NavComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  constructor() {}

  ngOnInit(): void {}
}
