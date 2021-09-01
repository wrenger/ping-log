import React from 'react';
import api from './api';

interface MCServersProps {
    servers: api.MCServer[],
}

export class MCServers extends React.Component<MCServersProps> {
    render() {
        return (
            <div>
                {this.props.servers.map((s, i) => (
                    <div className="stats" key={i}>
                        <table>
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
                    </div>
                ))}
            </div>
        );
    }
}
