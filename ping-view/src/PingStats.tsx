import React from "react";
import { Card, Elevation } from "@blueprintjs/core";

import api from "./api";

export default class PingStats extends React.Component<api.HistoryData> {
    render() {
        return (
            <Card elevation={Elevation.TWO} className="small-box">
                <h5 className="bp4-heading">Ping</h5>
                <div className="center">
                    <span>
                        <span className="bp4-text-disabled">min: </span>
                        <span className="bp4-text-large">{this.props.min.toPrecision(3)}</span>
                    </span>
                    <span className="stats-avg">
                        <span className="bp4-text-disabled">avg: </span>
                        <span className="bp4-text-large">{this.props.avg.toPrecision(3)}</span>
                    </span>
                    <span>
                        <span className="bp4-text-disabled">max: </span>
                        <span className="bp4-text-large">{this.props.max.toPrecision(3)}</span>
                    </span>
                </div>
                <div className="center">
                    <span className="bp4-text-disabled">lost: </span>
                    <span className="bp4-text-large">{this.props.lost}</span>
                </div>
            </Card>
        );
    }
}
