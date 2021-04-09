import React, { useEffect } from 'react';
import { useRouter } from 'next/router';

import { notification } from 'antd';
import useAuth from '../lib/hooks/UseAuth';
import Layout from '../components/Layout';

export default function LogoutPage() {
  const { signout } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!router.isReady) {
      return;
    }

    signout()
      .then((_) => notification.open({ message: 'successfully logged out' }))
      .then((_) => router.push('/login'));
  }, [router.isReady]);

  return <Layout />;
}
