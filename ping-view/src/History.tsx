import React from 'react';
import moment from 'moment';
import { Line } from 'react-chartjs-2';
import api from './api';
import { iter, range } from './iter';

const HISTORY_CHART_LOG = {
    aspectRatio: 3,
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
    legend: {
        display: true
    },
    elements: {
        line: {
            tension: 0, // disables bezier curves
        }
    }
};

interface HistoryProps {
    pings: api.PingData[],
}

interface HistoryState {
    day: number,
}

export class History extends React.Component<HistoryProps, HistoryState> {
    constructor(props: HistoryProps) {
        super(props);
        this.state = {
            day: 0,
        };
    }

    onDayChange(e: React.ChangeEvent<HTMLSelectElement>) {
        this.setState({ day: Number.parseInt(e.target.value) })
    }

    render() {
        const day = moment().subtract(this.state.day, "day");
        const begin = day.startOf("day").toDate();
        const end = day.endOf("day").toDate();

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
            <div className="box">
                <h2>Daily</h2>
                <select name="select-day" value={this.state.day} onChange={this.onDayChange.bind(this)}>
                    {
                        range(0, 7).map(i => (
                            <option value={i} key={i}>
                                {moment().subtract(i, "day").format("dd DD.MM.YYYY")}
                            </option>
                        ))
                    }
                </select>
                <div>
                    <Line className="chart"
                        options={HISTORY_CHART_LOG}
                        data={data} />
                </div>
            </div>
        );
    }
}
