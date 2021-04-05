import { useRouter } from 'next/router';
import { useEffect } from 'react';
import useAuth from '../lib/hooks/UseAuth';
import Layout from './Layout';


export default function AuthedLayout({ children, ...props }) {
    const router = useRouter();
    const { user } = useAuth();

    useEffect(() => {
        if (!user) {
            router.push("/login")
        }
    }, [user])

    return user ? <Layout {...props}>{children}</Layout> : <Layout></Layout>
}
