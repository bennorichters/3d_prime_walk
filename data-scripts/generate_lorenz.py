#!/usr/bin/env python3
"""
Generate Lorenz Attractor coordinates for 3D visualization.

The Lorenz system is a set of differential equations that exhibit chaotic behavior:
    dx/dt = σ(y - x)
    dy/dt = x(ρ - z) - y
    dz/dt = xy - βz

Classic parameters: σ=10, ρ=28, β=8/3
"""

def lorenz_attractor(steps=25000, dt=0.01):
    """
    Generate Lorenz attractor coordinates using Euler method.

    Args:
        steps: Number of points to generate
        dt: Time step for numerical integration

    Returns:
        List of (x, y, z) tuples
    """
    # Lorenz parameters (classic values)
    sigma = 10.0
    rho = 28.0
    beta = 8.0 / 3.0

    # Initial conditions (slightly off from origin to start the chaos)
    x, y, z = 0.1, 0.0, 0.0

    coordinates = []

    # Scale factor to make the attractor fit nicely in view
    scale = 10.0

    for _ in range(steps):
        # Store current position (scaled)
        coordinates.append((x * scale, y * scale, z * scale))

        # Compute derivatives
        dx_dt = sigma * (y - x)
        dy_dt = x * (rho - z) - y
        dz_dt = x * y - beta * z

        # Euler integration step
        x += dx_dt * dt
        y += dy_dt * dt
        z += dz_dt * dt

    return coordinates

def main():
    print("Generating Lorenz Attractor coordinates...")

    # Generate 25,000 points
    coordinates = lorenz_attractor(steps=25000, dt=0.01)

    # Write to data file
    with open('data', 'w') as f:
        for x, y, z in coordinates:
            f.write(f"{x:.6f},{y:.6f},{z:.6f}\n")

    print(f"Generated {len(coordinates)} coordinates")
    print("Saved to 'data' file")
    print("\nRun with: cargo run --release 25000 255,0,0 0,0,255 data_walk")

if __name__ == "__main__":
    main()
