import {Component} from '@angular/core';
import {CommonModule} from '@angular/common';
import {RouterModule, RouterOutlet} from '@angular/router';
import {NavComponent} from './component/nav/nav.component';

@Component({
  selector: 'app-root',
  imports: [CommonModule, RouterOutlet, RouterModule, NavComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  constructor() {}

  ngOnInit(): void {}
}
