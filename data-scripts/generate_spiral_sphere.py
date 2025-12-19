#!/usr/bin/env python3

import math

def spiral(steps, radius, turns):
    coordinates = []

    y = -radius
    y_step = (2 * radius) / steps
    
    turn_angle = 0
    turn_angle_step = turns * 2 * math.pi / steps

    for i in range(steps):
        y_radius = math.sqrt(radius * radius - y * y)
        x = math.cos(turn_angle) * y_radius
        z = math.sin(turn_angle) * y_radius

        coordinates.append((x, y, z))
        y += y_step
        turn_angle += turn_angle_step

    return coordinates

def main():
    coordinates = spiral(steps=25000, radius=200, turns=30)

    with open('data', 'w') as f:
        for x, y, z in coordinates:
            f.write(f"{x:.6f},{y:.6f},{z:.6f}\n")

    print(f"Generated {len(coordinates)} coordinates")

if __name__ == "__main__":
    main()

