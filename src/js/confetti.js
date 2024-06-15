import canvasConfetti from 'https://cdn.jsdelivr.net/npm/canvas-confetti@1.9.3/+esm'

export function fireConfetti() {
    canvasConfetti({
        particleCount: 100,
        spread: 70,
        origin: {y: 0.6},
    });
}
