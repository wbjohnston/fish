import { Card, Table, Typography } from 'antd'
import { useRouter } from 'next/router'
import { useEffect, useState } from 'react';
import Layout from '../../components/AuthedLayout'
import { fetchGame } from '../../lib/api';
import useAuth from '../../lib/hooks/UseAuth';

export default function ListGamePage() {
    const router = useRouter();
    const { user } = useAuth();
    const [game, setGame] = useState({});
    const [websocket, setWebsocket] = useState(null);

    useEffect(() => {
        if (!router.isReady) {
            return
        }
        fetchGame(router.query.id).then(setGame)
    }, [])


    useEffect(() => {
        const websocket = new WebSocket(`ws://localhost:8080/me/ws`)
        setWebsocket(websocket)
    }, [])

    return <Layout>
        <h1>Game: {game.name}</h1>

        <h2>RAW</h2>
        <Card>
            <pre>
                <Typography code>
                    {JSON.stringify(game, null, 2)}
                </Typography>
            </pre>
        </Card>
    </Layout>
}
