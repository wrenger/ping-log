import moment from "moment";
import { PeekableIterator } from "./iter";

namespace api {
    const API_LOG = "/api/pings";
    const API_HW = "/api/hw";
    const API_MC = "/api/mc";

    function baseUrl(): string {
        if ((globalThis as any).isDev)
            return "http://localhost:8081";
        else
            return "";
    }

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


    export async function pings(start: Date, end: Date, count: number): Promise<PingData[]> {
        const response = await fetch(encodeURI(baseUrl() + API_LOG + "?" + new URLSearchParams({
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

    export async function hardware(): Promise<HardwareData> {
        const response = await fetch(baseUrl() + API_HW);
        return await response.json();
    }

    export async function mcServers(): Promise<MCServer[]> {
        const response = await fetch(baseUrl() + API_MC);
        return await response.json();
    }

    export function stats(time: Date, pings: Iterable<PingData>): HistoryData {
        let min = 1000.0;
        let max = 0.0;
        let sum = 0.0;
        let lost = 0.0;
        let count = 0;

        for (const entry of pings) {
            if (entry.ping < min) {
                min = entry.ping;
            }
            if (entry.ping >= 1000.0) {
                lost += 1.0;
            } else {
                if (entry.ping > max) {
                    max = entry.ping;
                }
                sum += entry.ping;
            }
            count += 1;
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

    export function* statsIter(pings: PeekableIterator<PingData>): Generator<HistoryData> {
        let first = pings.peek();
        if (first.done) return;

        let current = moment(first.value.time).startOf("hour");
        while (!pings.peek().done) {
            let until = current.toDate();
            yield stats(until, pings.take(p => p.time > until));
            current = current.subtract(1, "hour");
        }
    }
}

export default api;
