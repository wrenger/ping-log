import React from "react";

import api from "./api";

export default class PingStats extends React.Component<api.HistoryData> {
    render() {
        return (
            <div className="card m-5">
                <div className="card-header">Ping</div>
                <div className="card-body">
                    <div className="row center text-center">
                        <div className="col">
                            <span className="text-secondary">min: </span>
                            <span>{this.props.min.toPrecision(3)}</span>
                        </div>
                        <div className="col">
                            <span className="text-secondary">avg: </span>
                            <span>{this.props.avg.toPrecision(3)}</span>
                        </div>
                        <div className="col">
                            <span className="text-secondary">max: </span>
                            <span>{this.props.max.toPrecision(3)}</span>
                        </div>
                    </div>
                    <div className="row">
                        <div className="col center">
                            <span className="text-secondary">lost: </span>
                            <span>{this.props.lost}</span>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}
