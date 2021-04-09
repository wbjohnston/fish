import React, { useState, useEffect } from 'react';

import axios from 'axios';
import { me } from '../lib/api';
import AuthedLayout from '../components/AuthedLayout';

export default function MePage({ initialMe }) {
  const [user, setUser] = useState(initialMe);

  useEffect(() => {
    me().then(setUser);
  }, []);

  return (
    <AuthedLayout>
      <h1>{user.username}</h1>
    </AuthedLayout>
  );
}

export async function getServerSideProps(context) {
  const response = await axios({
    url: 'http://localhost:8080/me',
    method: 'get',
    headers: context?.req?.headers?.cookie ? { cookie: context.req.headers.cookie } : undefined,
    withCredentials: true,
  });

  return {
    props: {
      initialMe: response.data,
    },
  };
}
