
import {Component, input} from '@angular/core';

@Component({
  selector: 'app-deviation',
  imports: [],
  templateUrl: './deviation.component.html',
  styleUrl: './deviation.component.scss',
})
export class DeviationComponent {
  // @Input({required: true}) deviation: number
  readonly deviation = input.required<number>();
  readonly active = input(true);
}
