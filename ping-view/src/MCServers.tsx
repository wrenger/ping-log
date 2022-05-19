import React from 'react';
import { Card, Elevation } from '@blueprintjs/core';

import api from './api';

interface MCServersProps {
    servers: api.MCServer[],
}

export class MCServers extends React.Component<MCServersProps> {
    render() {
        return (
            <div>
                {this.props.servers.map((s, i) => (
                    <Card elevation={Elevation.TWO} className="small-box" key={i}>
                        <h5 className="bp4-heading">Minecraft</h5>
                        <table className="bp4-html-table bp4-html-table-condensed full-width">
                            <thead>
                                <tr>
                                    <th>{s.addr}</th>
                                    <th>{s.version}</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>{s.description}</td>
                                    <td>{s.players}/{s.max_players}</td>
                                </tr>
                            </tbody>
                        </table>
                    </Card>
                ))}
            </div>
        );
    }
}
