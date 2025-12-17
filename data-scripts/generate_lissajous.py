#!/usr/bin/env python3
"""
Generate 3D Lissajous Curve coordinates for 3D visualization.

Lissajous curves combine sinusoidal motions in three dimensions with different
frequencies and phase shifts, creating beautiful flowing patterns.

Parametric equations:
    x(t) = A·sin(a·t + δx)
    y(t) = B·sin(b·t + δy)
    z(t) = C·sin(c·t + δz)

Where:
    a, b, c = frequency ratios (integers create closed curves)
    δx, δy, δz = phase shifts in radians
    A, B, C = amplitudes (scale in each dimension)
"""

import argparse
import math

def generate_lissajous(a=3, b=4, c=5,
                       phase_x=0.0, phase_y=math.pi/2, phase_z=0.0,
                       amp_x=100.0, amp_y=100.0, amp_z=100.0,
                       steps=25000):
    """
    Generate 3D Lissajous curve coordinates.

    Args:
        a, b, c: Frequency ratios for x, y, z axes
        phase_x, phase_y, phase_z: Phase shifts in radians
        amp_x, amp_y, amp_z: Amplitudes (scale) for each axis
        steps: Number of points to generate

    Returns:
        List of (x, y, z) tuples
    """
    coordinates = []

    # Calculate period to complete the curve
    # LCM of frequencies determines when curve closes
    from math import gcd
    def lcm(a, b):
        return abs(a * b) // gcd(a, b)

    period = lcm(lcm(a, b), c)
    t_max = 2 * math.pi * period

    for i in range(steps):
        t = (i / steps) * t_max

        # Lissajous parametric equations
        x = amp_x * math.sin(a * t + phase_x)
        y = amp_y * math.sin(b * t + phase_y)
        z = amp_z * math.sin(c * t + phase_z)

        coordinates.append((x, y, z))

    return coordinates

def main():
    parser = argparse.ArgumentParser(
        description='Generate 3D Lissajous curve coordinates for visualization.',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Default beautiful pattern (3:4:5 frequencies)
  python3 generate_lissajous.py

  # Simple circle (equal frequencies, no phase)
  python3 generate_lissajous.py --a 1 --b 1 --c 1 --phase-x 0 --phase-y 0 --phase-z 0

  # Sphere (equal frequencies with phases)
  python3 generate_lissajous.py --a 1 --b 1 --c 1

  # Flowing ribbon pattern
  python3 generate_lissajous.py --a 1 --b 2 --c 3

  # Complex woven structure
  python3 generate_lissajous.py --a 3 --b 5 --c 7

  # Rose-like pattern
  python3 generate_lissajous.py --a 2 --b 3 --c 5

  # Adjust size
  python3 generate_lissajous.py --amp 150

  # Different amplitudes per axis (stretched)
  python3 generate_lissajous.py --amp-x 150 --amp-y 100 --amp-z 80

Popular frequency ratios:
  (1,1,1) - Circle/sphere
  (1,2,2) - Figure-8 pattern
  (1,2,3) - Flowing ribbon
  (2,3,4) - Woven pattern
  (3,4,5) - Complex beauty (default)
  (3,5,7) - Intricate knot
  (5,6,7) - Dense weave

Phase shifts (in radians):
  0       - No shift
  π/4     - Quarter shift (0.785398)
  π/2     - Half shift (1.570796)
  π       - Full shift (3.141593)
        """
    )

    parser.add_argument('--a', type=int, default=3,
                        help='X-axis frequency (default: 3)')
    parser.add_argument('--b', type=int, default=4,
                        help='Y-axis frequency (default: 4)')
    parser.add_argument('--c', type=int, default=5,
                        help='Z-axis frequency (default: 5)')

    parser.add_argument('--phase-x', type=float, default=0.0,
                        help='X-axis phase shift in radians (default: 0.0)')
    parser.add_argument('--phase-y', type=float, default=math.pi/2,
                        help=f'Y-axis phase shift in radians (default: π/2 = {math.pi/2:.6f})')
    parser.add_argument('--phase-z', type=float, default=0.0,
                        help='Z-axis phase shift in radians (default: 0.0)')

    parser.add_argument('--amp', type=float,
                        help='Amplitude for all axes (overrides individual settings)')
    parser.add_argument('--amp-x', type=float, default=100.0,
                        help='X-axis amplitude (default: 100.0)')
    parser.add_argument('--amp-y', type=float, default=100.0,
                        help='Y-axis amplitude (default: 100.0)')
    parser.add_argument('--amp-z', type=float, default=100.0,
                        help='Z-axis amplitude (default: 100.0)')

    parser.add_argument('--steps', type=int, default=25000,
                        help='Number of points to generate (default: 25000)')
    parser.add_argument('--output', type=str, default='data',
                        help='Output filename (default: data)')

    args = parser.parse_args()

    # If --amp is specified, use it for all axes
    if args.amp is not None:
        args.amp_x = args.amp_y = args.amp_z = args.amp

    # Validate frequencies are positive
    if args.a <= 0 or args.b <= 0 or args.c <= 0:
        parser.error("Frequencies (a, b, c) must be positive integers")

    print(f"Generating 3D Lissajous Curve ({args.a}:{args.b}:{args.c})...")
    print(f"  Frequencies: a={args.a}, b={args.b}, c={args.c}")
    print(f"  Phases: δx={args.phase_x:.4f}, δy={args.phase_y:.4f}, δz={args.phase_z:.4f}")
    print(f"  Amplitudes: Ax={args.amp_x}, Ay={args.amp_y}, Az={args.amp_z}")
    print(f"  Steps: {args.steps}")

    # Generate coordinates
    coordinates = generate_lissajous(
        a=args.a, b=args.b, c=args.c,
        phase_x=args.phase_x, phase_y=args.phase_y, phase_z=args.phase_z,
        amp_x=args.amp_x, amp_y=args.amp_y, amp_z=args.amp_z,
        steps=args.steps
    )

    # Write to file
    with open(args.output, 'w') as f:
        for x, y, z in coordinates:
            f.write(f"{x:.6f},{y:.6f},{z:.6f}\n")

    print(f"Generated {len(coordinates)} coordinates")
    print(f"Saved to '{args.output}'")
    print(f"\nRun with: cargo run --release {args.steps} 255,0,0 0,0,255 data_walk")

if __name__ == "__main__":
    main()
