import { useRouter } from 'next/router'
import { useEffect, useState } from 'react';
import { fetchClient } from '../../lib/api';


export default function FetchClient() {
    const router = useRouter();

    const { id } = router.query;

    const [client, setClient] = useState({});

    useEffect(() => {
        if (!router.isReady) {
            return
        }
        fetchClient(id).then(x => setClient(x))
    }, [id])


    return <div>{JSON.stringify(client)}</div>
}
