import api from "./api";
import { iter } from "./iter";

test("history", () => {


    let log: api.PingData[] = [
        { time: new Date(1536062893000), ping: 10.0 }, { time: new Date(1536059293000), ping: 20.0 }
    ];

    let history = [...api.statsIter(iter(log.values()))];

    console.log(history);

    expect(history.length).toBe(2);
    expect(history[0])
        .toBe({ time: new Date(1536066000000), min: 10.0, max: 10.0, avg: 10.0, lost: 0.0, count: 1 });
    expect(history[1])
        .toBe({ time: new Date(1536062400000), min: 20.0, max: 20.0, avg: 20.0, lost: 0.0, count: 1 });

    // let log = [Ping:: new (1536062893, 10.0), Ping:: new (1536055693, 20.0)];

    // let history = generate_history(& log);
    // println!("{:?}", history);

    // assert_eq!(3, history.len());
    // assert_eq!(
    //     History:: new (1536066000, 10.0, 10.0, 10.0, 0.0, 1),
    //     history[0]
    // );
    // assert_eq!(History:: new (1536062400, 0.0, 0.0, 0.0, 0.0, 0), history[1]);
    // assert_eq!(
    //     History:: new (1536058800, 20.0, 20.0, 20.0, 0.0, 1),
    //     history[2]
    // );
});
