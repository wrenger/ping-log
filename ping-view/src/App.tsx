import * as React from 'react';

import api from "./api";
import moment from "moment";
import { Hardware } from "./Hardware";
import PingStats from "./PingStats";
import { Pings } from "./Pings";
import { MCServers } from "./MCServers";
import { History } from "./History";

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
    const pings = this.state.pings;
    const until = moment().subtract(1, "hour").toDate();
    const untilIdx = pings.findIndex(p => p.time <= until);
    const stats = api.stats(until, pings.slice(0, untilIdx).map(p => p.ping));

    return (
      <div className="App">
        <h1 style={{ textAlign: "center" }}>Ping Log</h1>
        <div className="container" style={{ maxWidth: "28rem" }}>
          <PingStats {...stats} />
          <MCServers servers={this.state.mcServers} />
        </div>
        <div className="container">
          <Pings pings={pings} />
          <History pings={pings} />
        </div>
        <div className="container" style={{ maxWidth: "28rem" }}>
          <Hardware {...this.state.hardware} />
        </div>
        <button
          type="button"
          className="btn btn-primary reload"
          onClick={this.reload.bind(this)}
          title="Refresh"
        >↻</button>
      </div>
    );
  }
}


export default App;
