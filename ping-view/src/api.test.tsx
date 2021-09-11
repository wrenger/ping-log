import api from "./api";
import { iter } from "./iter";

test("iter", () => {
    let values = [0, 1, 2, 3];
    let v = iter(values.values());
    expect([...v]).toStrictEqual(values);

    v = iter(values.values());
    expect([...v.skip(_ => true)]).toStrictEqual([]);
    v = iter(values.values());
    expect([...v.skip(_ => false)]).toStrictEqual(values);
    v = iter(values.values());
    expect([...v.skip(x => x < 2)]).toStrictEqual([2, 3]);

    v = iter(values.values());
    expect([...v.take(_ => true)]).toStrictEqual(values);
    v = iter(values.values());
    expect([...v.take(_ => false)]).toStrictEqual([]);
    v = iter(values.values());
    expect([...v.take(x => x < 2)]).toStrictEqual([0, 1]);

    v = iter(values.values());
    expect([...v.map(x => x < 2)]).toStrictEqual([true, true, false, false]);
});


test("history", () => {
    let log: api.PingData[] = [
        { time: new Date(1536062893000), ping: 10.0 }, { time: new Date(1536059293000), ping: 20.0 }
    ];
    let history = [...api.statsIter(iter(log.values()))];

    expect(history.length).toBe(2);
    expect(history[0])
        .toStrictEqual({ time: new Date(1536066000000), min: 10.0, max: 10.0, avg: 10.0, lost: 0.0, count: 1 });
    expect(history[1])
        .toStrictEqual({ time: new Date(1536062400000), min: 20.0, max: 20.0, avg: 20.0, lost: 0.0, count: 1 });

    log = [{ time: new Date(1536062893000), ping: 10.0 }, { time: new Date(1536055693000), ping: 20.0 }];
    history = [...api.statsIter(iter(log.values()))];

    expect(history.length).toBe(3);
    expect(history[0])
        .toStrictEqual({ time: new Date(1536066000000), min: 10.0, max: 10.0, avg: 10.0, lost: 0.0, count: 1 });
    expect(history[1])
        .toStrictEqual({ time: new Date(1536062400000), min: 0.0, max: 0.0, avg: 0.0, lost: 0.0, count: 0 });
    expect(history[2])
        .toStrictEqual({ time: new Date(1536058800000), min: 20.0, max: 20.0, avg: 20.0, lost: 0.0, count: 1 })
});
