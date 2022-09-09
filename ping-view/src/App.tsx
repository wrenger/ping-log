import {
    Chart as ChartJS, LineController, LineElement,
    PointElement, LinearScale, Title, TimeScale
} from "chart.js";
import 'chartjs-adapter-moment';

import React from 'react';
import { Button } from "@blueprintjs/core";
import moment from "moment";

import { Hardware } from './Hardware';
import { MCServers } from './MCServers';
import { Pings } from "./Pings";
import { History } from "./History";
import api from "./api";
import PingStats from "./PingStats";
import { iter } from "./iter";

ChartJS.register(LineController, LineElement, PointElement, LinearScale, Title, TimeScale);
ChartJS.defaults.color = "#eeeeee";
ChartJS.defaults.animation = false;

interface AppProps { }

interface AppState {
    pings: api.PingData[],
    mcServers: api.MCServer[],
    hardware: api.HardwareData,
}

export class App extends React.Component<AppProps, AppState> {

    constructor(props: AppProps) {
        super(props);

        this.state = {
            pings: [],
            mcServers: [],
            hardware: { load: 0, memory_used: 0, memory_total: 0, temperature: 0 },
        }
    }

    async reload() {
        let [pings, mcServers, hardware] = await Promise.all([
            api.pings(new Date(), moment().subtract(1, "month").startOf("day").toDate(), 32 * 24 * 60),
            api.mcServers(),
            api.hardware(),
        ]);

        this.setState({
            pings: pings,
            mcServers: mcServers,
            hardware: hardware,
        })
    }

    componentDidMount() {
        this.reload();
        setInterval(this.reload.bind(this), 30000);
    }

    render() {
        const until = moment().subtract(1, "hour").toDate();
        const stats = api.stats(until, iter(this.state.pings.values()).take(p => p.time > until));

        return (
            <div className="log-display">
                <PingStats {...stats} />
                <MCServers servers={this.state.mcServers} />
                <Pings pings={this.state.pings} />
                <History pings={this.state.pings} />
                <Hardware {...this.state.hardware} />
                <Button
                    className="reload bp4-icon-refresh"
                    onClick={this.reload.bind(this)}
                    title="Refresh"
                />
            </div>
        );
    }
}
