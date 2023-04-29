from queue import Queue, Empty
import threading
import subprocess


def battle(a, b):
    p = subprocess.run(
        map(str, ['./target/release/hpas', *a, *b]), 
        capture_output=True)
    return list(map(int, p.stdout.split()))


def worker(inq: Queue, outq: Queue):
    while True:
        task = inq.get()
        if task is None:
            return
        outq.put([task, battle(*task)])


def output_worker(q: Queue, end: threading.Event):
    while end.is_set():
        try:
            (a, b), r = q.get(timeout=1.0)
        except Empty:
            continue
        print(a[0], a[1], b[0], b[1], r[0], r[1], flush=True)


def make_one(delta):
    for a in range(5, 100, delta):
        for s in range(1, 100, delta):
            if a + s < 100:
                yield a, s


def main():
    inq = Queue(maxsize=1024)
    outq = Queue()

    workers = []
    for _ in range(6):
        t = threading.Thread(target=worker, args=(inq, outq))
        t.start()
        workers.append(t)
    end = threading.Event()
    end.set()
    ow = threading.Thread(target=output_worker, args=(outq, end))
    ow.start()

    done = set()
    delta = 1
    for a in make_one(delta):
        for b in make_one(delta):
            if a == b:
                continue
            if (a, b) in done:
                continue
            done.add((b, a))
            inq.put((a, b))
    for _ in workers:
        inq.put(None)
    for w in workers:
        w.join()
    end.clear()
    ow.join()


if __name__ == '__main__':
    main()
