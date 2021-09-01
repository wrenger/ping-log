import React from "react";
import api from "./api";

export default class PingStats extends React.Component<api.HistoryData> {
    render() {
        return (
            <div className="stats">
                <div>
                    <span className="stats-min">{this.props.min.toPrecision(2)}</span>
                    <span className="stats-avg">{this.props.avg.toPrecision(2)}</span>
                    <span className="stats-max">{this.props.max.toPrecision(2)}</span>
                </div>
                <div className="stats-lost">{this.props.lost}</div>
            </div>
        );
    }
}
