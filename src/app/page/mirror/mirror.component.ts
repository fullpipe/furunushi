import { Component, ElementRef, OnDestroy, ViewChild } from '@angular/core';

@Component({
  selector: 'app-mirror',
  imports: [],
  templateUrl: './mirror.component.html',
  styleUrl: './mirror.component.scss',
})
export class MirrorComponent implements OnDestroy {
  @ViewChild('mirrorElement') mirrorElement!: ElementRef<HTMLVideoElement>;
  mirroring = false;
  stream: MediaStream | undefined;

  async ngOnInit() {
    this.stream = await navigator.mediaDevices.getUserMedia({
      audio: false,
      video: {
        frameRate: { ideal: 25, max: 30 },
        autoGainControl: false,
        noiseSuppression: false,
        backgroundBlur: false,
        echoCancellation: false,
      },
    });

    this.mirrorElement.nativeElement.srcObject = this.stream;
    this.mirroring = true;
  }

  ngOnDestroy(): void {
    if (!this.stream) {
      return;
    }

    this.stream.getTracks().forEach(function (track) {
      track.stop();
    });
  }

  async mirror() {}
}
