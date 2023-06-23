import React from 'react';
import moment from 'moment';
import { CartesianGrid, Legend, Line, LineChart, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";

import api from "./api";

interface PingsProps {
    pings: api.PingData[],
}

export class Pings extends React.Component<PingsProps> {
    render() {
        const until = moment().subtract(1, "hour").toDate();
        const untilIdx = this.props.pings.findIndex(p => p.time < until);
        let pings = this.props.pings.slice(0, untilIdx);

        pings.reverse();
        const data = pings.map((e) => {
            return {
                ping: e.ping < 1000 ? e.ping : undefined,
                lost: e.ping < 1000 ? undefined : 1,
                time: e.time,
            }
        });

        return (
            <div className="card m-5">
                <div className="card-header">Recent</div>
                <div className="card-body">
                    <ResponsiveContainer aspect={2.5} maxHeight={320}>
                        <LineChart data={data}>
                            <CartesianGrid stroke="var(--bs-border-color)" />
                            <XAxis dataKey={(element) => moment(element.time).format("LT")}
                                stroke="var(--bs-body-color)" />
                            <YAxis yAxisId="left" stroke="var(--bs-body-color)" />
                            <YAxis yAxisId="right" orientation="right" stroke="var(--bs-body-color)"
                                domain={[0.0, 1.0]} />
                            <Tooltip isAnimationActive={false} contentStyle={{
                                width: "100px",
                                backgroundColor: "var(--bs-secondary-bg)",
                                border: "1px solid var(--bs-border-color)"
                            }} />
                            <Legend verticalAlign="top" />
                            <Line connectNulls yAxisId="left" isAnimationActive={false} dataKey="ping"
                                stroke="#4996fa" strokeWidth={3} />
                            <Line yAxisId="right" isAnimationActive={false} dataKey="lost"
                                stroke="#d85858" strokeWidth={3} />
                        </LineChart>
                    </ResponsiveContainer>
                </div>
            </div>
        );
    }
}
