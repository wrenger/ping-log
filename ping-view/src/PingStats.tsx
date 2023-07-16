import api from "./api";

export default function PingStats(stats: api.HistoryData) {
    return (
        <div className="card m-5">
            <div className="card-header">Ping</div>
            <div className="card-body">
                <div className="row center text-center">
                    <div className="col">
                        <span className="text-secondary">min: </span>
                        <span>{stats.min.toPrecision(3)}</span>
                    </div>
                    <div className="col">
                        <span className="text-secondary">avg: </span>
                        <span>{stats.avg.toPrecision(3)}</span>
                    </div>
                    <div className="col">
                        <span className="text-secondary">max: </span>
                        <span>{stats.max.toPrecision(3)}</span>
                    </div>
                </div>
                <div className="row">
                    <div className="col center">
                        <span className="text-secondary">lost: </span>
                        <span>{stats.lost}</span>
                    </div>
                </div>
            </div>
        </div>
    );
}
