import { Component, computed, effect, signal } from '@angular/core';
import { ControlValueAccessor, NG_VALUE_ACCESSOR } from '@angular/forms';
import { NoteComponent } from '../note/note.component';
import { NoteNameIdx } from '../../bindings/NoteNameIdx';

const MIN = 12;
const MAX = 88;

@Component({
  selector: 'app-note-input',
  templateUrl: './note-input.component.html',
  styleUrl: './note-input.component.scss',
  imports: [NoteComponent],
  providers: [
    {
      provide: NG_VALUE_ACCESSOR,
      multi: true,
      useExisting: NoteInputComponent,
    },
  ],
})
export class NoteInputComponent implements ControlValueAccessor {
  midi = signal(49);
  note = computed(() => {
    const n = this.midi() % 12;
    const octave = Math.floor(this.midi() / 12) - 1;

    return {
      name: NoteNameIdx[n],
      octave,
    };
  });

  onChange = (_: number) => {};
  onTouched = () => {};

  touched = false;
  disabled = false;

  constructor() {
    effect(() => this.onChange(this.midi()));
  }

  minus() {
    this.markAsTouched();
    if (!this.disabled && this.midi() > MIN) {
      this.midi.set(this.midi() - 1);
    }
  }

  plus() {
    this.markAsTouched();
    if (!this.disabled && this.midi() < MAX) {
      this.midi.set(this.midi() + 1);
    }
  }

  writeValue(midi: number) {
    this.midi.set(midi);
  }

  registerOnChange(onChange: any) {
    this.onChange = onChange;
  }

  registerOnTouched(onTouched: any) {
    this.onTouched = onTouched;
  }

  markAsTouched() {
    if (!this.touched) {
      this.onTouched();
      this.touched = true;
    }
  }

  setDisabledState(disabled: boolean) {
    this.disabled = disabled;
  }
}
