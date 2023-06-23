import moment from "moment";

namespace api {
    const API_LOG = "/api/pings";
    const API_HW = "/api/hw";
    const API_MC = "/api/mc";

    export interface HistoryData {
        time: Date,
        min: number,
        max: number,
        avg: number,
        lost: number,
        count: number,
    }

    export interface PingData {
        /** Send time of the ping request. */
        time: Date,
        /** Response time in milliseconds. */
        ping: number,
    }

    export interface HardwareData {
        /** CPU load in percent times the number of CPUs. */
        load: number,
        /** Current memory consumption. */
        memory_used: number,
        /** Total memory installed on the system. */
        memory_total: number,
        /** CPU temperature. */
        temperature: number,
    }

    export interface MCServer {
        addr: string,
        version: string,
        description: string,
        players: number,
        max_players: number,
    }

    function get<T>(obj: any, prop: string, def: T): T {
        if (obj instanceof Object && prop in obj && obj[prop] as T)
            return obj[prop];
        return def;
    }

    /** Fetch the most recent pings (latest first) */
    export async function pings(start: Date, end: Date, count: number): Promise<PingData[]> {
        const response = await fetch(encodeURI(API_LOG + "?" + new URLSearchParams({
            start: Math.round(start.getTime() / 1000.0).toString(),
            end: Math.round(end.getTime() / 1000.0).toString(),
            count: count.toString(),
        }).toString()));

        const parsed: any[] = await response.json();
        return parsed.map(p => {
            return {
                time: new Date(get<number>(p, "time", 0) * 1000.0),
                ping: get<number>(p, "ping", 0.0),
            }
        });
    }

    /** Fetch the hardware statistics. */
    export async function hardware(): Promise<HardwareData> {
        const response = await fetch(API_HW);
        return await response.json();
    }

    /** Fetch server status. */
    export async function mcServers(): Promise<MCServer[]> {
        const response = await fetch(API_MC);
        return await response.json();
    }

    /** Compute the combined statistic for all pings before the given time. */
    export function stats(time: Date, pings: number[]): HistoryData {
        let min = 1000.0;
        let max = 0.0;
        let sum = 0.0;
        let lost = 0.0;
        let count = pings.length;

        for (const ping of pings) {
            if (ping < min) {
                min = ping;
            }
            if (ping >= 1000.0) {
                lost += 1.0;
            } else {
                if (ping > max) {
                    max = ping;
                }
                sum += ping;
            }
        }

        let avg = Math.round(1000.0 * sum / (count - lost)) / 1000.0;
        lost = Math.round(1000.0 * lost / count) / 1000.0;

        return {
            time: time,
            min: min >= 1000.0 ? 0.0 : min,
            max: max <= 0.0 ? 0.0 : max,
            avg: isNaN(avg) ? 0.0 : avg,
            lost: isNaN(lost) ? 0.0 : lost,
            count: count,
        };
    }

    function group<T, K, V>(arr: T[], f: (e: T) => [K, V]): [K, V[]][] {
        let out: [K, V[]][] = [];
        for (const e of arr) {
            const [k, v] = f(e);
            let last = out.at(-1);
            if (last !== undefined && last[0] === k) {
                last[1].push(v);
            } else {
                out.push([k, [v]]);
            }

        }
        return out;
    }

    export function statsArray(pings: PingData[]): HistoryData[] {
        let chunks = group(pings, p => [moment(p.time).startOf("hour").toDate().getTime(), p.ping]);
        let out: HistoryData[] = []

        if (chunks.length > 0) {
            let i = 0;
            let time = new Date(chunks[0][0]);
            // Fill non-existing hours with 0
            while (i < chunks.length) {
                let [t, chunk] = chunks[i];
                if (t >= time.getTime()) {
                    out.push(stats(time, chunk));
                    i += 1;
                } else {
                    out.push({ time: time, min: 0, max: 0, avg: 0, lost: 0, count: 0 });
                }
                time = moment(time).subtract(1, "hour").toDate();
            }
        }

        return out;
    }
}

export default api;
