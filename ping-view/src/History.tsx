import React from 'react';
import moment from 'moment';
import { Line } from 'react-chartjs-2';
import { ChartOptions } from 'chart.js';
import { Button, Card, Elevation } from '@blueprintjs/core';
import { DatePicker } from '@blueprintjs/datetime';
import { Popover2 } from '@blueprintjs/popover2';

import api from './api';
import { iter } from './iter';

const HISTORY_CHART_LOG: ChartOptions<"line"> = {
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
    date: Date,
    isOpen: boolean,
}

export class History extends React.Component<HistoryProps, HistoryState> {
    constructor(props: HistoryProps) {
        super(props);
        this.state = {
            date: new Date(),
            isOpen: false,
        };
    }

    private onDateChange(date: Date, userChange: boolean) {
        // Sometime the date can be null
        if (!date) date = this.state.date;
        this.setState({ date: date, isOpen: !userChange })
    }

    private dateInteraction(nextOpenState: boolean) {
        this.setState({ isOpen: nextOpenState })
    }

    render() {
        const day = moment(this.state.date);
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
            <Card elevation={Elevation.TWO} className="box">
                <h5 className="bp4-heading row">
                    <span className="stretch">Daily</span>
                    <Popover2
                        interactionKind="click"
                        isOpen={this.state.isOpen}
                        onInteraction={state => this.dateInteraction(state)}
                        content={
                            <DatePicker
                                value={this.state.date}
                                minDate={moment().subtract(1, "month").toDate()}
                                maxDate={new Date()}
                                onChange={(newDate, userChange) => this.onDateChange(newDate, userChange)}
                            />
                        }>
                        <Button
                            className="bp4-icon-calendar"
                            text={this.state.date.toLocaleDateString()}
                        />
                    </Popover2>
                </h5>
                <Line className="chart"
                    options={HISTORY_CHART_LOG}
                    data={data} />
            </Card>
        );
    }
}
