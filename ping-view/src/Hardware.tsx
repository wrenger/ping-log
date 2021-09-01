import React from 'react';
import api from './api';

export class Hardware extends React.Component<api.HardwareData> {
    render() {
        return (
            <div className="stats selectable">
                <div title="Sum of all cores (400% means 4 cores with 100%)"
                >CPU Load: {this.props.load.toPrecision(2)}%</div>
                <div title="Used / Total">Memory: {this.props.memory_used.toPrecision(2)} GB / {this.props.memory_total.toPrecision(2)} GB</div>
                <div>Temperature: {this.props.temperature.toPrecision(2)}°</div>
            </div>
        );
    }
}
