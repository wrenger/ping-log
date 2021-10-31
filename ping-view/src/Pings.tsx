import React from 'react';
import moment from 'moment';
import { Line } from 'react-chartjs-2';
import { ChartOptions } from 'chart.js';

import api from "./api";
import { iter } from './iter';

interface PingsProps {
    pings: api.PingData[],
}

const RECENT_CHART_OPTIONS: ChartOptions<"line"> = {
    aspectRatio: 3,
    scales: {
        ms: {
            type: "linear",
            position: "left",
            beginAtZero: true,
        },
        lost: {
            type: "linear",
            beginAtZero: true,
            ticks: {
                precision: 0,
            },
            position: "right",
            grid: {
                drawOnChartArea: false, // only want the grid lines for one axis to show up
            },
        },
        x: {
            type: "time",
            offset: true,
            time: {
                minUnit: "second"
            }
        }
    },
    elements: {
        line: {
            tension: 0, // disables bezier curves
        }
    }
};

export class Pings extends React.Component<PingsProps> {
    render() {
        const until = moment().subtract(1, "hour").toDate();
        let pings = [...iter(this.props.pings.values()).take(p => p.time > until)];

        let labels: Date[] = [];
        let times: number[] = [];
        let lost: number[] = [];
        pings.reverse();
        pings.forEach(element => {
            labels.push(element.time);
            if (element.ping < 1000) {
                times.push(element.ping);
                lost.push(0);
            } else {
                times.push(0);
                lost.push(1);
            }
        });

        let data = {
            labels: labels,
            datasets: [{
                label: "Ping",
                data: times,
                borderColor: "#4996fa",
                backgroundColor: "#4996fa",
                fill: false,
                yAxisID: 'ms'
            }, {
                label: "Lost",
                data: lost,
                borderColor: "#d85858",
                backgroundColor: "#d85858",
                fill: false,
                yAxisID: 'lost'
            }],
        };

        return (
            <article className="box">
                <header>Recent</header>
                <section>
                    <Line className="chart"
                        options={RECENT_CHART_OPTIONS}
                        data={data} />
                </section>
                <footer></footer>
            </article>
        );
    }
}
