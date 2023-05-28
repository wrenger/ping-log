import React from 'react';

import api from './api';

interface MCServersProps {
    servers: api.MCServer[],
}

export class MCServers extends React.Component<MCServersProps> {
    render() {
        return (
            <>
                {this.props.servers.map((s, i) => (
                    <div key={i}>
                        <MCServer server={s} />
                    </div>
                ))}
            </>
        );
    }
}


interface MCServerProps {
    server: api.MCServer
}

class MCServer extends React.Component<MCServerProps> {
    render() {
        const s = this.props.server;
        return (
            <div className="card m-5">
                <div className="card-header">Minecraft</div>
                <div className="card-body">
                    <table className="full-width">
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
            </div>
        )
    }
}
