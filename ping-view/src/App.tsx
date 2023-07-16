import * as React from 'react';

import api from "./api";
import moment from "moment";
import { Hardware } from "./Hardware";
import PingStats from "./PingStats";
import { Pings } from "./Pings";
import { MCServers } from "./MCServers";
import { History } from "./History";

export default function App() {
  const [pings, setPings] = React.useState<api.PingData[]>([]);
  const [mcServers, setMcServers] = React.useState<api.MCServer[]>([]);
  const [hardware, setHardware] = React.useState<api.HardwareData>(
    { load: 0, memory_used: 0, memory_total: 0, temperature: 0 });

  var loading = false;

  async function reload() {
    if (loading) return;

    loading = true;
    let [p, m, h] = await Promise.all([
      api.pings(new Date(), moment().subtract(1, "month").startOf("day").toDate(), 32 * 24 * 60),
      api.mcServers(),
      api.hardware(),
    ]);

    setPings(p);
    setMcServers(m);
    setHardware(h);
    loading = false;
  }

  React.useEffect(() => {
    reload();
    let timer = setInterval(() => { reload(); }, 30000);
    return () => clearInterval(timer);
    // eslint-disable-next-line
  }, []);

  const until = moment().subtract(1, "hour").toDate();
  const untilIdx = pings.findIndex(p => p.time <= until);
  const stats = api.stats(until, pings.slice(0, untilIdx).map(p => p.ping));

  return (
    <div className="App">
      <h1 style={{ textAlign: "center" }}>Ping Log</h1>
      <div className="container" style={{ maxWidth: "28rem" }}>
        <PingStats {...stats} />
        <MCServers servers={mcServers} />
      </div>
      <div className="container">
        <Pings pings={pings} />
        <History pings={pings} />
      </div>
      <div className="container" style={{ maxWidth: "28rem" }}>
        <Hardware {...hardware} />
      </div>
      <button type="button" className="btn btn-primary reload" onClick={reload} title="Refresh">↻</button>
    </div>
  );
}
