import { Table } from 'antd'
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
    </Layout>
}
