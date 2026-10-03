"""Reproduce Timber Harbor's original PCM16 music and delivery chime using Python 3."""
import math
from pathlib import Path
import random
import struct
import wave

RATE = 22050
ROOT = Path(__file__).resolve().parent


def note(midi):
    """Convert a MIDI note number to frequency."""
    return 440 * 2 ** ((midi - 69) / 12)


def pluck(frequency, age, duration):
    """Synthesize a soft wooden mallet with a short attack and exponential tail."""
    if age < 0 or age >= duration:
        return 0
    envelope = min(age / 0.008, 1) * math.exp(-age * 4 / duration)
    phase = 2 * math.pi * frequency * age
    return envelope * (math.sin(phase) + 0.24 * math.sin(phase * 2) + 0.07 * math.sin(phase * 3))


def write(name, values):
    """Write an engine-decodable mono PCM16 WAV."""
    with wave.open(str(ROOT / name), "wb") as output:
        output.setparams((1, 2, RATE, 0, "NONE", "not compressed"))
        output.writeframes(b"".join(struct.pack("<h", round(max(-1, min(1, value)) * 28000)) for value in values))


def music():
    """Create a seamless 24-second harbor theme in C major with a gentle tide bed."""
    chords = [(48, 52, 55, 59), (45, 48, 52, 55), (41, 45, 48, 52), (43, 47, 50, 55)]
    events = []
    for beat in range(48):
        chord = chords[(beat // 12) % 4]
        events.append((beat * 0.5, note(chord[beat % 4] + 12), 0.23, 1.4))
        if beat % 3 == 0:
            events.append((beat * 0.5, note(chord[0]), 0.15, 2.8))
    rng = random.Random(18)
    filtered = 0
    values = []
    for sample in range(24 * RATE):
        time = sample / RATE
        filtered = 0.97 * filtered + 0.03 * rng.uniform(-1, 1)
        value = filtered * (0.025 + 0.012 * math.sin(time * math.pi / 3))
        for start, frequency, volume, duration in events:
            age = (time - start) % 24
            if age < duration:
                value += volume * pluck(frequency, age, duration)
        values.append(value)
    write("music.wav", values)


def delivery():
    """Create an ascending three-note export confirmation."""
    values = []
    for sample in range(RATE):
        time = sample / RATE
        values.append(sum(0.25 * pluck(note(midi), time - index * 0.09, 0.6) for index, midi in enumerate((72, 76, 79))))
    write("pickup.wav", values)


if __name__ == "__main__":
    music()
    delivery()
