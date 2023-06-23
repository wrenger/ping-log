import React from 'react';

import api from './api';

export class Hardware extends React.Component<api.HardwareData> {
    render() {
        return (
            <div className="card m-5">
                <div className="card-header">Hardware</div>
                <div className="card-body">
                    <table className="full-width">
                        <tbody>
                            <tr>
                                <td className="td-label text-secondary">CPU Load: </td>
                                <td>{this.props.load.toPrecision(3)}%</td>
                            </tr>
                            <tr title="Used / Total">
                                <td className="td-label text-secondary">Memory: </td>
                                <td>{this.props.memory_used.toPrecision(3)} / {this.props.memory_total.toPrecision(3)} GB</td>
                            </tr>
                            <tr>
                                <td className="td-label text-secondary">Temperature: </td>
                                <td>{this.props.temperature.toPrecision(3)}°</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        );
    }
}
