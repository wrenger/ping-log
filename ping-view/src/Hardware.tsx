import React from 'react';

import api from './api';

export function Hardware(data: api.HardwareData) {
    return (
        <div className="card m-5">
            <div className="card-header">Hardware</div>
            <div className="card-body">
                <table className="full-width">
                    <tbody>
                        <tr>
                            <td className="td-label text-secondary">CPU Load: </td>
                            <td>{data.load.toPrecision(3)}%</td>
                        </tr>
                        <tr title="Used / Total">
                            <td className="td-label text-secondary">Memory: </td>
                            <td>{data.memory_used.toPrecision(3)} / {data.memory_total.toPrecision(3)} GB</td>
                        </tr>
                        <tr>
                            <td className="td-label text-secondary">Temperature: </td>
                            <td>{data.temperature.toPrecision(3)}°</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    );
}
