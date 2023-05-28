import {
  Chart as ChartJS, LineController, LineElement,
  PointElement, LinearScale, Title, TimeScale,
  Tooltip, Legend,
} from "chart.js";
import 'chartjs-adapter-moment';

import * as React from 'react';

import api from "./api";
import moment from "moment";
import { iter } from "./iter";
import { Hardware } from "./Hardware";
import PingStats from "./PingStats";
import { Pings } from "./Pings";
import { MCServers } from "./MCServers";
import { History } from "./History";

ChartJS.register(
  LineController, LineElement, PointElement, LinearScale,
  Legend, Title, TimeScale, Tooltip,
);
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
      <div className="App">
        <h1 style={{ textAlign: "center" }}>Ping Log</h1>
        <div className="container" style={{ maxWidth: "24rem" }}>
          <PingStats {...stats} />
          <MCServers servers={this.state.mcServers} />
        </div>
        <div className="container">
          <Pings pings={this.state.pings} />
          <History pings={this.state.pings} />
        </div>
        <div className="container" style={{ maxWidth: "24rem" }}>
          <Hardware {...this.state.hardware} />
        </div>
        <button
          type="button"
          className="btn btn-primary btn-sm reload"
          onClick={this.reload.bind(this)}
          title="Refresh"
        >↻</button>
      </div>
    );
  }
}


export default App;
