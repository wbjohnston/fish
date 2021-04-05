import Layout from '../components/Layout'
import { useRouter } from 'next/router';
import { useEffect } from 'react';
import useAuth from '../lib/hooks/UseAuth';



export default function LogoutPage() {
    const router = useRouter();
    const { signout } = useAuth();


    useEffect(() => {
        if (!router.isReady) {
            return
        }

        signout().then(_ => router.push("/login"))
    }, [])


    return <Layout></Layout>

}
