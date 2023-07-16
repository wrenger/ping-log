import api from './api';

export function MCServers({ servers }: { servers: api.MCServer[] }) {
    return (
        <>
            {servers.map((s, i) => (
                <div key={i}>
                    <MCServer server={s} />
                </div>
            ))}
        </>
    );
}

function MCServer({ server }: { server: api.MCServer }) {
    return (
        <div className="card m-5">
            <div className="card-header">Minecraft</div>
            <div className="card-body">
                <table className="full-width">
                    <thead>
                        <tr>
                            <th>{server.addr}</th>
                            <th>{server.version}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>{server.description}</td>
                            <td>{server.players}/{server.max_players}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    )
}
