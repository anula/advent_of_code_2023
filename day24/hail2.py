# %%
import sys
from dataclasses import dataclass
import random
import numpy as np

# %%
from typing import List

@dataclass
class XYZ:
    x: int
    y: int
    z: int

    @property
    def coords(self):
        return [self.x, self.y, self.z]

    def __add__(self, o):
        return XYZ(x = self.x + o.x, y = self.y + o.y, z = self.z + o.z)

    def __mul__(self, scalar):
        return XYZ(x = self.x * scalar, y = self.y * scalar, z = self.z * scalar)
        
    def __sub__(self, o):
        return self + (o * -1)

DOWN    = XYZ( 0, -1,  0)
UP      = XYZ( 0,  1,  0)
LEFT    = XYZ(-1,  0,  0)
RIGHT   = XYZ( 1,  0,  0)
BACK    = XYZ( 0,  0, -1)
FORWARD = XYZ( 0,  0,  1)

# %%
def parse_XYZ(line):
    parts = [int(x.strip()) for x in line.strip().split(', ')]
    return XYZ(*parts)

# %%
@dataclass
class Ray:
    start: XYZ
    velocity: XYZ

def parse_Ray(line):
    parts = line.strip().split(' @ ')
    return Ray(start=parse_XYZ(parts[0]), velocity=parse_XYZ(parts[1]))

flat_order = [BACK, RIGHT, FORWARD, LEFT]
side_order = [LEFT, BACK, RIGHT, FORWARD]

def flat_yield(start, size):
    curr = start
    yield curr
    step_size = 1
    ord_idx = 0
    while step_size <= size:
        for _a in range(0, 2):
            for _b in range(0, step_size):
                curr += flat_order[ord_idx]
                yield curr
            ord_idx += 1
            ord_idx %= 4
        step_size += 1
    for _a in range(0, size):
        curr += flat_order[ord_idx]
        yield curr


def cube_points_iterator(max_size=None, start_size=2):
    size = start_size
    curr = XYZ(0, 1, 0) * (size // 2)
    while True:
        if max_size is not None and max_size < size:
            break
        top_point = curr + UP

        # Flat top
        yield from flat_yield(top_point, size)

        # Sides
        front_right_point = top_point + FORWARD * (size // 2) + RIGHT * (size // 2)
        curr = front_right_point
        for side in range(1, size):
            curr += DOWN
            yield curr
            for ord_idx in range(0, 4):
                for _i in range(1, size + 1):
                    curr += side_order[ord_idx]
                    yield curr

        # Flat bottom
        bottom_point = top_point + (DOWN * (size))
        yield from flat_yield(bottom_point, size)

        # End
        curr = top_point
        print('Past all points in cube of size: ', size)
        size += 2

def simpler_iterator(min_x, max_x, min_y, max_y, min_z, max_z):
    for x in range(min_x, max_x + 1):
        for y in range(min_y, max_y + 1):
            for z in range(min_z, max_z + 1):
                yield XYZ(x, y, z)

# %%
# 0: s.x, 1: s.y, 2: s.z, 3: t_0, .... 

def equations_for(co_len, velocity, coord, rays: List[Ray], how_many):
    coefficients = []
    results = []
    for i in range(0, min(how_many, len(rays))):
        ray = rays[i]
        cos = [0] * co_len
        cos[coord] = 1
        t_idx = 3 + i
        cos[t_idx] = velocity.coords[coord] - ray.velocity.coords[coord]
        coefficients.append(cos)
        results.append(ray.start.coords[coord])
    return (coefficients, results)

def compute_for(velocity, rays):
    coefficients = []
    results = []

    co_len = len(rays) + 3
    left = co_len*3
    e, r = equations_for(co_len, velocity, 0, rays, left)
    coefficients.extend(e)
    results.extend(r)

    left -= len(r)
    e, r = equations_for(co_len, velocity, 1, rays, left)
    coefficients.extend(e)
    results.extend(r)

    left -= len(r)
    e, r = equations_for(co_len, velocity, 2, rays, left)
    coefficients.extend(e)
    results.extend(r)

    c = np.array(coefficients)
    r = np.array(results)

    res, _, _, _ = np.linalg.lstsq(c, r)
    return res

# %%
def verify(res, velocity: XYZ, rays: List[Ray]):
    for coord_idx in range(0, 3):
        s = res[coord_idx]
        s_v = velocity.coords[coord_idx]
        for ray_idx, ray in enumerate(rays):
            t_idx = 3 + ray_idx
            t = res[t_idx]
            #print("l, r:", s + t* (s_v - ray.velocity.coords[coord_idx]), ray.start.coords[coord_idx])
            if not np.isclose(s + t* (s_v - ray.velocity.coords[coord_idx]), ray.start.coords[coord_idx], rtol=0.001, atol=0.001):
                return False
    return True

# %%
def find_solution(rays, num, iterator):
    for vel in iterator:
        sample_rays = random.sample(rays, num)
        res = compute_for(vel, sample_rays)
        if verify(res, vel, sample_rays):
            start = XYZ(res[0], res[1], res[2])
            print("Start: ", start)
            print("Velocity: ", vel)
            return

# %%

def main(start_size):
    print("For start: ", start_size)

    with open('input', 'r') as f:
        input_file = f.readlines()

    rays = [parse_Ray(line) for line in input_file]

    find_solution(rays, len(rays), start_size)


if __name__ == '__main__':
    main(start_size=int(sys.argv[1]))