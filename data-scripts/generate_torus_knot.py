#!/usr/bin/env python3
"""
Generate Torus Knot coordinates for 3D visualization.

A torus knot wraps around a torus (donut shape) in a beautiful pattern.
The (p, q) parameters determine the knot type:
    - (3, 2): Trefoil knot - the simplest true knot
    - (5, 2): Cinquefoil knot - five-petaled flower pattern
    - (7, 2): Septafoil knot - seven-petaled pattern
    - (5, 3): More complex interweaving
    - (8, 3): Beautiful rope-like structure

Parametric equations:
    x(t) = (R + r·cos(p·t)) · cos(q·t)
    y(t) = (R + r·cos(p·t)) · sin(q·t)
    z(t) = r·sin(p·t)
"""

import argparse
import math

def generate_torus_knot(p=3, q=2, R=100.0, r=50.0, steps=25000):
    """
    Generate torus knot coordinates.

    Args:
        p: Number of times the knot winds around the torus longitudinally
        q: Number of times the knot winds around the torus meridionally
        R: Major radius (distance from center to tube center)
        r: Minor radius (tube thickness)
        steps: Number of points to generate

    Returns:
        List of (x, y, z) tuples
    """
    coordinates = []

    # Parameter t goes from 0 to 2π·lcm(p,q)/q to complete the knot
    # For simplicity, we'll use a multiple of 2π that ensures completion
    t_max = 2 * math.pi * max(p, q)

    for i in range(steps):
        t = (i / steps) * t_max

        # Torus knot parametric equations
        x = (R + r * math.cos(p * t)) * math.cos(q * t)
        y = (R + r * math.cos(p * t)) * math.sin(q * t)
        z = r * math.sin(p * t)

        coordinates.append((x, y, z))

    return coordinates

def main():
    parser = argparse.ArgumentParser(
        description='Generate torus knot coordinates for 3D visualization.',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Trefoil knot (default)
  python3 generate_torus_knot.py

  # Cinquefoil knot
  python3 generate_torus_knot.py --p 5 --q 2

  # Complex interweaving pattern
  python3 generate_torus_knot.py --p 5 --q 3

  # Larger, thicker knot
  python3 generate_torus_knot.py --R 150 --r 80

  # Save to custom file
  python3 generate_torus_knot.py --output my_knot.dat

Popular knot types:
  (3,2) - Trefoil knot
  (5,2) - Cinquefoil knot
  (7,2) - Septafoil knot
  (5,3) - Complex weave
  (8,3) - Beautiful rope pattern
  (7,3) - Intricate structure
        """
    )

    parser.add_argument('--p', type=int, default=3,
                        help='Longitudinal wrapping number (default: 3)')
    parser.add_argument('--q', type=int, default=2,
                        help='Meridional wrapping number (default: 2)')
    parser.add_argument('--R', type=float, default=100.0,
                        help='Major radius - distance to tube center (default: 100.0)')
    parser.add_argument('--r', type=float, default=50.0,
                        help='Minor radius - tube thickness (default: 50.0)')
    parser.add_argument('--steps', type=int, default=25000,
                        help='Number of points to generate (default: 25000)')
    parser.add_argument('--output', type=str, default='data',
                        help='Output filename (default: data)')

    args = parser.parse_args()

    # Validate that p and q are positive
    if args.p <= 0 or args.q <= 0:
        parser.error("p and q must be positive integers")

    print(f"Generating ({args.p},{args.q}) Torus Knot...")
    print(f"  Major radius (R): {args.R}")
    print(f"  Minor radius (r): {args.r}")
    print(f"  Steps: {args.steps}")

    # Generate coordinates
    coordinates = generate_torus_knot(
        p=args.p,
        q=args.q,
        R=args.R,
        r=args.r,
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
