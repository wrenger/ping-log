import * as React from 'react';
import moment from 'moment';
import { LineChart, Line, CartesianGrid, XAxis, YAxis, Tooltip, Legend, ResponsiveContainer } from 'recharts';

import api from './api';

interface HistoryProps {
    pings: api.PingData[],
}

interface HistoryState {
    date: Date,
}

export class History extends React.Component<HistoryProps, HistoryState> {
    constructor(props: HistoryProps) {
        super(props);
        this.state = {
            date: new Date(),
        };
    }

    onDateChange(event: React.ChangeEvent<HTMLInputElement>) {
        this.setState({ date: new Date(event.target.value) });
    }

    render() {
        const day = moment(this.state.date);
        const begin = day.startOf("day").toDate();
        const end = day.endOf("day").toDate();
        const str = day.format("YYYY-MM-DD");

        let pingsBegin = this.props.pings.findIndex(p => p.time < end);
        if (pingsBegin < 0) pingsBegin = 0;
        let pingsEnd = this.props.pings.findIndex(p => p.time < begin);
        const pings = this.props.pings.slice(pingsBegin, pingsEnd);

        let history = api.statsArray(pings);
        history.reverse();

        return (
            <div className="card m-5">
                <div className="card-header">
                    <div className="row align-items-center">
                        <div className="col">
                            <span>Daily</span>
                        </div>
                        <div className="col col-auto">
                            <input type="date" value={str} onChange={(value) => this.onDateChange(value)} />
                        </div>
                    </div>
                </div>
                <div className="card-body">
                    <ResponsiveContainer aspect={2.5} maxHeight={320}>
                        <LineChart data={history}>
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
                            <Line yAxisId="left" isAnimationActive={false} dataKey="avg"
                                stroke="#4996fa" strokeWidth={3} />
                            <Line yAxisId="left" isAnimationActive={false} dataKey="min"
                                stroke="#58d878" strokeWidth={3} />
                            <Line yAxisId="left" isAnimationActive={false} dataKey="max"
                                stroke="#d8d658" strokeWidth={3} />
                            <Line yAxisId="right" isAnimationActive={false} dataKey="lost"
                                stroke="#d85858" strokeWidth={3} />
                        </LineChart>
                    </ResponsiveContainer>
                </div>
            </div>
        );
    }
}
