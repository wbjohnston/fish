import { useRouter } from 'next/router'
import { useEffect, useState } from 'react';
import Layout from '../../components/AuthedLayout';
import { fetchClient } from '../../lib/api';


export default function FetchClient() {
    const router = useRouter();
    const [client, setClient] = useState({});

    useEffect(() => {
        if (!router.isReady) {
            return
        }
        fetchClient(router.query.id).then(setClient)
    }, [])



    return <Layout>
        <h1>{client.name}</h1>
    </Layout>
}
