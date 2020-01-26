import subprocess


def battle(a, b):
    p = subprocess.run(
        map(str, ['./target/release/hpas', *a, *b]), 
        capture_output=True)
    return list(map(int, p.stdout.split()))


def make_one(delta):
    for a in range(delta, 100, delta):
        for s in range(delta, 100, delta):
            if a + s < 100:
                yield a, s


def main():
    done = set()
    for a in make_one(5):
        for b in make_one(5):
            if a == b:
                continue
            if (a, b) in done:
                continue
            done.add((b, a))
            res = battle(a, b)
            print(a[0], a[1], b[0], b[1], res[0], res[1])


if __name__ == '__main__':
    main()
