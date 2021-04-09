import React, { useEffect } from 'react';
import { useRouter } from 'next/router';

import useAuth from '../lib/hooks/UseAuth';
import Layout from './Layout';

export default function AuthedLayout({ children, ...props }) {
  const { user } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!user && router.isReady) {
      router.push('/login');
    }
  }, [user, router.isReady]);

  return user ? <Layout {...props}>{children}</Layout> : <Layout />;
}
