import React, { useState, useEffect } from 'react';

import { me } from '../lib/api';
import AuthedLayout from '../components/AuthedLayout';

export default function MePage() {
  const [user, setUser] = useState({});

  useEffect(() => {
    me().then(setUser);
  }, [user]);

  return (
    <AuthedLayout>
      <h1>{user.username}</h1>
    </AuthedLayout>
  );
}
