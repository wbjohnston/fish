import { Table } from 'antd'
import { useRouter } from 'next/router'
import { useEffect, useState } from 'react';
import Layout from '../../components/AuthedLayout'
import { fetchGame } from '../../lib/api';

export default function ListGamePage() {
    const router = useRouter();
    const [game, setGame] = useState({});

    useEffect(() => {
        if (!router.isReady) {
            return
        }
        fetchGame(router.query.id).then(setGame)
    }, [])


    return <Layout>
        <h1>{game.name}</h1>
    </Layout>
}
