import * as React from 'react';
import moment from 'moment';
import { Line } from 'react-chartjs-2';
import { ChartOptions } from 'chart.js';

import api from './api';
import { iter } from './iter';

const HISTORY_CHART_LOG: ChartOptions<"line"> = {
    maintainAspectRatio: false,
    scales: {
        ms: {
            type: "linear",
            beginAtZero: true,
            position: "left"
        },
        lost: {
            type: "linear",
            beginAtZero: true,
            ticks: {
                precision: 0,
            },
            position: "right"
        },
        x: {
            type: "time",
            offset: true
        }
    },
    elements: {
        line: {
            tension: 0, // disables bezier curves
        }
    },
};

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
        console.log(event.target.value);
        this.setState({ date: new Date(event.target.value) });
    }

    render() {
        const day = moment(this.state.date);
        const begin = day.startOf("day").toDate();
        const end = day.endOf("day").toDate();
        const str = day.format("YYYY-MM-DD");

        const pings = iter(this.props.pings.values()).skip(p => p.time > end).take(p => p.time > begin);
        let history: api.HistoryData[] = [...api.statsIter(pings)];

        let labels: Date[] = [];
        let dataAvg: number[] = [];
        let dataMin: number[] = [];
        let dataMax: number[] = [];
        let dataLost: number[] = [];
        history.reverse();
        for (const element of history) {
            labels.push(element.time);
            dataAvg.push(element.avg);
            dataMin.push(element.min);
            dataMax.push(element.max);
            dataLost.push(element.lost);
        }

        const data = {
            labels: labels,
            datasets: [{
                label: "Avg",
                data: dataAvg,
                borderColor: "#4996fa",
                backgroundColor: "#4996fa",
                fill: false,
                yAxisID: 'ms'
            }, {
                label: "Min",
                data: dataMin,
                borderColor: "#58d878",
                backgroundColor: "#58d878",
                fill: false,
                yAxisID: 'ms'
            }, {
                label: "Max",
                data: dataMax,
                borderColor: "#d8d658",
                backgroundColor: "#d8d658",
                fill: false,
                yAxisID: 'ms'
            }, {
                label: "Lost",
                data: dataLost,
                borderColor: "#d85858",
                backgroundColor: "#d85858",
                fill: false,
                yAxisID: 'lost'
            }],
        }

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
                    <div className="chart-container">
                        <Line className="chart"
                            options={HISTORY_CHART_LOG}
                            width="100%"
                            data={data} />
                    </div>
                </div>
            </div>
        );
    }
}
