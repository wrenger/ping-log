import React from "react";
import { Card, Elevation } from "@blueprintjs/core";

import api from "./api";

export default class PingStats extends React.Component<api.HistoryData> {
    render() {
        return (
            <Card elevation={Elevation.TWO} className="small-box">
                <h5 className="bp4-heading">Ping</h5>
                <div className="selectable">
                    <div>
                        <span className="stats-min">{this.props.min.toPrecision(2)}</span>
                        <span className="stats-avg">{this.props.avg.toPrecision(2)}</span>
                        <span className="stats-max">{this.props.max.toPrecision(2)}</span>
                    </div>
                    <div className="stats-lost">{this.props.lost}</div>
                </div>
            </Card>
        );
    }
}
