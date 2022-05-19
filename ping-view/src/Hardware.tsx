import React from 'react';
import { Card, Elevation } from '@blueprintjs/core';

import api from './api';

export class Hardware extends React.Component<api.HardwareData> {
    render() {
        return (
            <Card elevation={Elevation.TWO} className="small-box">
                <h5 className="bp4-heading">Hardware</h5>
                <table className="full-width">
                    <tbody>
                        <tr>
                            <td className="bp4-text-disabled td-label">CPU Load: </td>
                            <td>{this.props.load.toPrecision(3)}%</td>
                        </tr>
                        <tr title="Used / Total">
                            <td className="bp4-text-disabled td-label">Memory: </td>
                            <td>{this.props.memory_used.toPrecision(3)} / {this.props.memory_total.toPrecision(3)} GB</td>
                        </tr>
                        <tr>
                            <td className="bp4-text-disabled td-label">Temparature: </td>
                            <td>{this.props.temperature.toPrecision(3)}°</td>
                        </tr>
                    </tbody>
                </table>
            </Card>
        );
    }
}
