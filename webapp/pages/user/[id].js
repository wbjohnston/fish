import { Card, Table, Typography } from 'antd'
import { useRouter } from 'next/router'
import { useEffect, useState } from 'react';
import Layout from '../../components/AuthedLayout'
import { fetchUser } from '../../lib/api';

export default function ListGamePage() {
    const router = useRouter();
    const [user, setUser] = useState({});

    useEffect(() => {
        if (!router.isReady) {
            return
        }
        fetchUser(router.query.id).then(setUser)
    }, [])


    return <Layout>
        <h1>{user.username}</h1>

        <Card title="Raw JSON">
            <pre>
                <Typography code>
                    {JSON.stringify(user, null, 2)}
                </Typography>
            </pre>

        </Card>
    </Layout>
}
