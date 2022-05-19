import React from 'react';
import { Card, Elevation } from '@blueprintjs/core';

import api from './api';

export class Hardware extends React.Component<api.HardwareData> {
    render() {
        return (
            <Card elevation={Elevation.TWO} className="small-box">
                <h5 className="bp4-heading">Hardware</h5>
                <div className="selectable">
                    <div title="Sum of all cores (400% means 4 cores with 100%)"
                    >CPU Load: {this.props.load.toPrecision(3)}%</div>
                    <div title="Used / Total">Memory: {this.props.memory_used.toPrecision(3)} GB / {this.props.memory_total.toPrecision(3)} GB</div>
                    <div>Temperature: {this.props.temperature.toPrecision(3)}°</div>
                </div>
            </Card>
        );
    }
}
