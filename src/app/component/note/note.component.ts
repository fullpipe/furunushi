import { Component, input } from '@angular/core';
import { Note } from '../../bindings/Note';

@Component({
  selector: 'app-note',
  imports: [],
  templateUrl: './note.component.html',
  styleUrl: './note.component.scss',
})
export class NoteComponent {
  readonly note = input<Pick<Note, 'name' | 'octave'>>();
}
