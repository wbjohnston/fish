import { Button, Card, Table, Typography } from 'antd'
import { useRouter } from 'next/router'
import { useEffect, useState } from 'react';
import Layout from '../../components/AuthedLayout'
import { fetchGame } from '../../lib/api';
import useAuth from '../../lib/hooks/UseAuth';
import useWebsocket from '../../lib/hooks/UseWebsocket';

export default function ListGamePage() {
    const router = useRouter();
    const { user } = useAuth();
    const [game, setGame] = useState({});
    const { websocket } = useWebsocket();

    useEffect(() => {
        if (!router.isReady) {
            return
        }
        fetchGame(router.query.id).then(setGame)
    }, [])


    function sitGame() {
        if (!websocket) {
            // throw error
        }

        websocket.send(JSON.stringify({
            gameId: router.query.id,
            action: {
                kind: 'sit',
                options: {}
            }
        }))

    }

    function leaveGame() {
        if (!websocket) {
            // throw error
        }

        websocket.send(JSON.stringify({
            gameId: router.query.id,
            action: {
                kind: 'leave'
            }
        }))
    }

    function joinGame() {
        if (!websocket) {
            // throw error
        }

        websocket.send(JSON.stringify({
            gameId: router.query.id,
            action: {
                kind: 'join'
            }
        }))
    }

    return <Layout>
        <h1>Game: {game.name}</h1>

        <Button onClick={joinGame} color="green">Join Game</Button>
        <Button onClick={sitGame} color="green">Sit</Button>
        <Button onClick={leaveGame} color="green">Leave </Button>

        <h2>RAW</h2>
        <Card>
            <pre>
                <Typography code="json">
                    {JSON.stringify(game, null, 2)}
                </Typography>
            </pre>
        </Card>
    </Layout>
}
