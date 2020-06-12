import re

commands = ["M", "v", "h"]

def to_commands(s):
    regex = r"([a-zA-Z])\s?([0-9-,]*)"
    res = []

    for m in re.finditer(regex, s):
        res.append((m.group(1), m.group(2)))

    return res

def to_points(commands):
    points = []
    for c in commands:
        letter = c[0]

        if letter == "M":
            x, y = c[1].split(",")
            points.append((int(x), int(y)))
        elif letter == "v":
            x, y = points[-1]
            arg = int(c[1])
            points.append( (x, y+arg) )
        elif letter == "h":
            x, y = points[-1]
            arg = int(c[1])
            points.append( (x+arg, y) )

    return points

def get_points():
    with open("fractal_string.txt") as f:
        s = f.read()

    c = to_commands(s)
    points = to_points(c)
    return points

def scale_points(points, scale):
    return [(scale * x, scale * y) for x, y in points]

def print_points_svg(points):
    if len(points) == 0: return ""

    first_x, first_y = points[0]
    l = ["M", "{},{}".format(first_x, first_y)]
    for point in points[1:]:
        l.append("L")
        l.append("{},{}".format(point[0], point[1]))

    return "".join(l)

def print_points_rust(points):
    it = map(lambda point: "Point::new({}, {})".format(point[0], point[1]), points)

    return ",\n".join(it)
